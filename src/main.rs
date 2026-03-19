use anyhow::{anyhow, Result};
use clap::Parser;

use ruth_cli::cli::{self, Cli, Commands};
use ruth_cli::config::Config;
use ruth_cli::gpg;
use ruth_cli::qr;
use ruth_cli::store::{Entry, Store};
use ruth_cli::totp;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let config = Config::resolve(cli.config, cli.gpg_id, None);

    match cli.command {
        Commands::Add(args) => cmd_add(&config, args),
        Commands::Get(args) => cmd_get(&config, &args.label),
        Commands::List => cmd_list(&config),
        Commands::Rm(args) => cmd_rm(&config, &args.label),
    }
}

fn cmd_add(config: &Config, args: cli::AddArgs) -> Result<()> {
    gpg::check_gpg()?;
    let gpg_key = config.require_gpg_key_id()?;
    config.ensure_config_dir()?;

    let mut store = load_or_create_store(config)?;

    let entry = if let Some(qr_path) = &args.qr {
        let data = qr::from_qr_image(qr_path)?;
        let label = args.label.unwrap_or_else(|| data.auto_label());
        if label.is_empty() {
            return Err(anyhow!(
                "Could not auto-generate label from QR code. Use --label to specify one."
            ));
        }
        Entry {
            label,
            domain: data.issuer.clone().unwrap_or_default(),
            account: data.account.clone().unwrap_or_default(),
            secret: data.secret,
            algorithm: data.algorithm.parse()?,
            digits: data.digits,
            period: data.period,
        }
    } else {
        let label = args.label.ok_or_else(|| anyhow!("--label is required"))?;
        Entry {
            label,
            domain: args.domain.unwrap_or_default(),
            account: args.account.unwrap_or_default(),
            secret: args.secret.ok_or_else(|| anyhow!("--secret is required"))?,
            algorithm: args.algorithm.parse()?,
            digits: args.digits,
            period: args.period,
        }
    };

    let label = entry.label.clone();
    store.add(entry)?;
    save_store(config, gpg_key, &store)?;
    eprintln!("Added entry '{}'. Use `ruth-cli get {}` to get the code.", label, label);
    Ok(())
}

fn cmd_get(config: &Config, label: &str) -> Result<()> {
    gpg::check_gpg()?;
    let store = load_store(config)?;
    let entry = store
        .find(label)
        .ok_or_else(|| anyhow!("label '{}' not found", label))?;
    let code = totp::generate_now(&entry.secret, entry.period, entry.digits, entry.algorithm)?;
    println!("{}", code);
    Ok(())
}

fn cmd_list(config: &Config) -> Result<()> {
    gpg::check_gpg()?;
    let store = load_store(config)?;
    for label in store.labels() {
        println!("{}", label);
    }
    Ok(())
}

fn cmd_rm(config: &Config, label: &str) -> Result<()> {
    gpg::check_gpg()?;
    let gpg_key = config.require_gpg_key_id()?;
    let mut store = load_store(config)?;
    store.remove(label)?;
    save_store(config, gpg_key, &store)?;
    Ok(())
}

fn load_store(config: &Config) -> Result<Store> {
    if !config.store_path.exists() {
        return Err(anyhow!(
            "No store found at {}. Run `ruth-cli add` first.",
            config.store_path.display()
        ));
    }
    let plaintext = gpg::decrypt(&config.store_path)?;
    Store::from_toml(&plaintext)
}

fn load_or_create_store(config: &Config) -> Result<Store> {
    if config.store_path.exists() {
        let plaintext = gpg::decrypt(&config.store_path)?;
        Store::from_toml(&plaintext)
    } else {
        Ok(Store::default())
    }
}

fn save_store(config: &Config, gpg_key: &str, store: &Store) -> Result<()> {
    let plaintext = store.to_toml()?;
    gpg::encrypt(&plaintext, gpg_key, &config.store_path)
}
