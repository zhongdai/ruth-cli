use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::totp::{self, Algorithm};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Entry {
    pub label: String,
    pub domain: String,
    pub account: String,
    pub secret: String,
    #[serde(default)]
    pub algorithm: Algorithm,
    #[serde(default = "default_digits")]
    pub digits: u32,
    #[serde(default = "default_period")]
    pub period: u64,
}

fn default_digits() -> u32 {
    6
}

fn default_period() -> u64 {
    30
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Store {
    #[serde(default, rename = "entry")]
    entries: Vec<Entry>,
}

impl Store {
    pub fn from_toml(s: &str) -> Result<Self> {
        toml::from_str(s).map_err(|e| anyhow!("failed to parse store: {}", e))
    }

    pub fn to_toml(&self) -> Result<String> {
        toml::to_string_pretty(self).map_err(|e| anyhow!("failed to serialize store: {}", e))
    }

    pub fn add(&mut self, entry: Entry) -> Result<()> {
        totp::validate_secret(&entry.secret)?;
        if self.entries.iter().any(|e| e.label == entry.label) {
            return Err(anyhow!(
                "label '{}' already exists. Use `ruth-cli rm {}` first.",
                entry.label,
                entry.label
            ));
        }
        self.entries.push(entry);
        Ok(())
    }

    pub fn remove(&mut self, label: &str) -> Result<()> {
        let len = self.entries.len();
        self.entries.retain(|e| e.label != label);
        if self.entries.len() == len {
            return Err(anyhow!("label '{}' not found", label));
        }
        Ok(())
    }

    pub fn find(&self, label: &str) -> Option<&Entry> {
        self.entries.iter().find(|e| e.label == label)
    }

    pub fn labels(&self) -> Vec<&str> {
        self.entries.iter().map(|e| e.label.as_str()).collect()
    }

    pub fn entries(&self) -> &[Entry] {
        &self.entries
    }
}
