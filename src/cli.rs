use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "ruth-cli",
    about = "Agent-friendly TOTP authenticator CLI",
    long_about = "ruth-cli generates time-based one-time passwords (TOTP, RFC 6238) from \
GPG-encrypted secrets stored in ~/.config/ruth-cli/store.toml.gpg.\n\n\
Non-interactive by design — all input via flags, env vars, or config files. \
Outputs only the TOTP code to stdout, making it ideal for scripts and AI agents.\n\n\
Setup:\n  \
1. Configure GPG key: echo 'gpg_key_id = \"you@example.com\"' > ~/.config/ruth-cli/config.toml\n  \
2. Add an entry:     ruth-cli add -l github -d github.com -a me@example.com -s JBSWY3DPEHPK3PXP\n  \
3. Get a code:       ruth-cli get github\n  \
4. Use in scripts:   TOKEN=$(ruth-cli get github)",
    version
)]
pub struct Cli {
    /// Path to the encrypted store file [env: RUTH_STORE] [default: ~/.config/ruth-cli/store.toml.gpg]
    #[arg(long, global = true)]
    pub config: Option<PathBuf>,

    /// GPG key ID (email, long key ID, or fingerprint) [env: RUTH_GPG_ID] [default: from ~/.config/ruth-cli/config.toml]
    #[arg(long, global = true)]
    pub gpg_id: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a new TOTP entry from a QR code image or manual input.
    ///
    /// From QR:    ruth-cli add -q /path/to/qr.png
    /// Manual:     ruth-cli add -l github -d github.com -a me@example.com -s JBSWY3DPEHPK3PXP
    ///
    /// On success, prints the stored label so you know what to pass to `get`.
    Add(AddArgs),

    /// Print the current TOTP code for an entry. Outputs only the code to stdout.
    ///
    /// Example:    ruth-cli get github
    /// In script:  TOKEN=$(ruth-cli get github)
    Get(GetArgs),

    /// List all stored entries (label, domain, account) in aligned columns.
    List,

    /// Remove an entry by its label.
    ///
    /// Example:    ruth-cli rm github
    Rm(RmArgs),
}

#[derive(clap::Args)]
pub struct AddArgs {
    /// Path to a QR code image (PNG/JPEG) containing an otpauth:// URI.
    /// All fields are auto-extracted. Mutually exclusive with --secret.
    #[arg(short, long, conflicts_with_all = ["secret"])]
    pub qr: Option<PathBuf>,

    /// Label for the entry (used with `get` and `rm`).
    /// Required for manual entry. Auto-generated from QR issuer/account if omitted.
    #[arg(short, long)]
    pub label: Option<String>,

    /// Domain or service name (e.g. github.com)
    #[arg(short, long, required_unless_present = "qr")]
    pub domain: Option<String>,

    /// Account identifier (e.g. user@example.com)
    #[arg(short, long, required_unless_present = "qr")]
    pub account: Option<String>,

    /// Base32-encoded TOTP secret
    #[arg(short, long, required_unless_present = "qr")]
    pub secret: Option<String>,

    /// HMAC algorithm
    #[arg(short = 'A', long, default_value = "SHA1", value_parser = ["SHA1", "SHA256", "SHA512"])]
    pub algorithm: String,

    /// Number of digits in the generated code
    #[arg(short = 'n', long, default_value = "6")]
    pub digits: u32,

    /// Time step period in seconds
    #[arg(short, long, default_value = "30")]
    pub period: u64,
}

#[derive(clap::Args)]
pub struct GetArgs {
    /// Label of the entry (as shown by `ruth-cli list`)
    pub label: String,
}

#[derive(clap::Args)]
pub struct RmArgs {
    /// Label of the entry to remove (as shown by `ruth-cli list`)
    pub label: String,
}
