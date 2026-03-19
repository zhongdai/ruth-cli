use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "ruth-cli", about = "Agent-friendly TOTP authenticator")]
pub struct Cli {
    /// Path to the encrypted store file
    #[arg(long, global = true)]
    pub config: Option<PathBuf>,

    /// GPG key ID for encryption
    #[arg(long, global = true)]
    pub gpg_id: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a new TOTP entry
    Add(AddArgs),
    /// Get the current TOTP code for a label
    Get(GetArgs),
    /// List all stored labels
    List,
    /// Remove an entry by label
    Rm(RmArgs),
}

#[derive(clap::Args)]
pub struct AddArgs {
    /// Path to a QR code image
    #[arg(short, long, conflicts_with_all = ["secret"])]
    pub qr: Option<PathBuf>,

    /// Label for the entry
    #[arg(short, long)]
    pub label: Option<String>,

    /// Domain / service name
    #[arg(short, long, required_unless_present = "qr")]
    pub domain: Option<String>,

    /// Account identifier (e.g. email)
    #[arg(short, long, required_unless_present = "qr")]
    pub account: Option<String>,

    /// Base32-encoded secret
    #[arg(short, long, required_unless_present = "qr")]
    pub secret: Option<String>,

    /// HMAC algorithm (SHA1, SHA256, SHA512)
    #[arg(short = 'A', long, default_value = "SHA1")]
    pub algorithm: String,

    /// Number of digits in the code
    #[arg(short = 'n', long, default_value = "6")]
    pub digits: u32,

    /// Time period in seconds
    #[arg(short, long, default_value = "30")]
    pub period: u64,
}

#[derive(clap::Args)]
pub struct GetArgs {
    /// Label of the entry
    pub label: String,
}

#[derive(clap::Args)]
pub struct RmArgs {
    /// Label of the entry to remove
    pub label: String,
}
