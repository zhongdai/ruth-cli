# ruth-cli

Agent-friendly TOTP authenticator CLI. Generates time-based one-time passwords (RFC 6238) from GPG-encrypted secrets. Non-interactive — designed for automation and AI agent workflows.

## Install

### From GitHub Releases

Download the latest binary for your platform from [Releases](../../releases).

### From Source

```bash
cargo install --path .
```

## Setup

1. Configure your GPG key ID (one of):

   ```bash
   # Option A: config file (recommended)
   # macOS: ~/Library/Application Support/ruth-cli/
   # Linux: ~/.config/ruth-cli/
   CONFIG_DIR="${XDG_CONFIG_HOME:-${HOME}/.config}/ruth-cli"  # Linux
   # CONFIG_DIR="${HOME}/Library/Application Support/ruth-cli"  # macOS
   mkdir -p "$CONFIG_DIR"
   echo 'gpg_key_id = "your-key@example.com"' > "$CONFIG_DIR/config.toml"

   # Option B: environment variable
   export RUTH_GPG_ID="your-key@example.com"
   ```

2. Add your first entry:

   ```bash
   # From a QR code image
   ruth-cli add --qr /path/to/qr.png

   # Manual entry
   ruth-cli add --label github --domain github.com --account me@example.com --secret JBSWY3DPEHPK3PXP
   ```

## Usage

```bash
# Get a TOTP code
ruth-cli get github
# Output: 482937

# Use in scripts
TOKEN=$(ruth-cli get github)

# List all entries
ruth-cli list

# Remove an entry
ruth-cli rm github
```

## Commands

| Command | Description |
|---------|------------|
| `add --qr <path>` | Add entry from QR code image |
| `add --label <l> --secret <s> ...` | Add entry manually |
| `get <label>` | Print current TOTP code |
| `list` | Print all labels |
| `rm <label>` | Remove an entry |

## Add Options

| Flag | Default | Description |
|------|---------|------------|
| `--qr <path>` | | QR code image (PNG/JPEG) |
| `--label <name>` | auto from QR | Entry label |
| `--domain <domain>` | | Service domain |
| `--account <account>` | | Account identifier |
| `--secret <base32>` | | TOTP secret |
| `--algorithm <alg>` | SHA1 | SHA1, SHA256, or SHA512 |
| `--digits <n>` | 6 | Code length |
| `--period <secs>` | 30 | Time step in seconds |

## Global Flags

| Flag | Env Var | Description |
|------|---------|------------|
| `--config <path>` | `RUTH_STORE` | Path to encrypted store |
| `--gpg-id <key>` | `RUTH_GPG_ID` | GPG recipient key ID |

## Storage

Secrets are stored in an encrypted file at the OS config directory, encrypted with your GPG key. The decrypted data is never written to disk.

| OS | Store path |
|----|-----------|
| macOS | `~/Library/Application Support/ruth-cli/store.toml.gpg` |
| Linux | `~/.config/ruth-cli/store.toml.gpg` |

Override with `--config <path>` or `RUTH_STORE` env var.

## Requirements

- [GPG](https://gnupg.org/) installed and a key pair configured
- A working `gpg-agent` for passphrase caching (optional but recommended)
