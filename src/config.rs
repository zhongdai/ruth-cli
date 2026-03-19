use anyhow::{anyhow, Result};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct ConfigFile {
    gpg_key_id: Option<String>,
}

#[derive(Debug)]
pub struct Config {
    pub store_path: PathBuf,
    pub gpg_key_id: Option<String>,
}

impl Config {
    pub fn resolve(
        flag_store: Option<PathBuf>,
        flag_gpg_id: Option<String>,
        config_file_path: Option<PathBuf>,
    ) -> Self {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("~/.config"))
            .join("ruth-cli");

        let store_path = flag_store
            .or_else(|| std::env::var("RUTH_STORE").ok().map(PathBuf::from))
            .unwrap_or_else(|| config_dir.join("store.toml.gpg"));

        let gpg_key_id = flag_gpg_id
            .or_else(|| std::env::var("RUTH_GPG_ID").ok())
            .or_else(|| {
                let cf = config_file_path.unwrap_or_else(|| config_dir.join("config.toml"));
                Self::read_config_file(&cf).and_then(|c| c.gpg_key_id)
            });

        Config {
            store_path,
            gpg_key_id,
        }
    }

    fn read_config_file(path: &std::path::Path) -> Option<ConfigFile> {
        let content = std::fs::read_to_string(path).ok()?;
        toml::from_str(&content).ok()
    }

    pub fn ensure_config_dir(&self) -> Result<()> {
        let dir = self
            .store_path
            .parent()
            .ok_or_else(|| anyhow!("invalid store path"))?;
        if !dir.exists() {
            std::fs::create_dir_all(dir)?;
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                std::fs::set_permissions(dir, std::fs::Permissions::from_mode(0o700))?;
            }
        }
        Ok(())
    }

    pub fn require_gpg_key_id(&self) -> Result<&str> {
        self.gpg_key_id.as_deref().ok_or_else(|| {
            anyhow!(
                "No GPG key ID configured. Set it via:\n  \
                 --gpg-id <key_id>\n  \
                 RUTH_GPG_ID env var\n  \
                 gpg_key_id in ~/.config/ruth-cli/config.toml"
            )
        })
    }
}
