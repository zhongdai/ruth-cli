# ruth-cli

[![Release](https://img.shields.io/github/v/release/zhongdai/ruth-cli)](https://github.com/zhongdai/ruth-cli/releases)
[![Build](https://github.com/zhongdai/ruth-cli/actions/workflows/release.yml/badge.svg)](https://github.com/zhongdai/ruth-cli/actions/workflows/release.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Homebrew](https://img.shields.io/badge/homebrew-zhongdai%2Ftap-orange)](https://github.com/zhongdai/homebrew-tap)

Agent-friendly TOTP authenticator CLI. Generates time-based one-time passwords (RFC 6238) from GPG-encrypted secrets. Non-interactive — designed for automation and AI agent workflows.

## Install

### Homebrew (macOS / Linux)

```bash
brew tap zhongdai/tap
brew install ruth-cli
```

### From GitHub Releases

Download the latest binary for your platform from [Releases](../../releases).

### Via Cargo

```bash
cargo install --git https://github.com/zhongdai/ruth-cli.git
```

### From Source

```bash
git clone https://github.com/zhongdai/ruth-cli.git
cd ruth-cli
cargo install --path .
```

## Setup

1. Configure your GPG key ID (one of):

   ```bash
   # Option A: config file (recommended)
   mkdir -p ~/.config/ruth-cli
   echo 'gpg_key_id = "your-key@example.com"' > ~/.config/ruth-cli/config.toml

   # Option B: environment variable
   export RUTH_GPG_ID="your-key@example.com"
   ```

   See [GPG Setup Guide](docs/GPG_SETUP.md) for installing GPG, creating keys, and finding your key ID.

2. Add your first entry:

   ```bash
   # From a QR code image (auto-extracts all fields)
   ruth-cli add -q /path/to/qr.png

   # Manual entry
   ruth-cli add -l github -d github.com -a me@example.com -s JBSWY3DPEHPK3PXP
   ```

   On success, ruth-cli prints the stored label:
   ```
   Added entry 'github'. Use `ruth-cli get github` to get the code.
   ```

## Usage

```bash
# Get a TOTP code (prints only the code to stdout)
ruth-cli get github
# Output: 482937

# Use in scripts — capture the code
TOKEN=$(ruth-cli get github)

# List all entries (label, domain, account)
ruth-cli list
# Output:
# github       github.com       me@example.com
# aws-prod     aws.amazon.com   admin@company.com

# Remove an entry
ruth-cli rm github
```

## Commands

| Command | Description |
|---------|------------|
| `add` | Add a new TOTP entry from QR code or manual input |
| `get <label>` | Print the current TOTP code to stdout |
| `list` | List all entries with label, domain, and account |
| `rm <label>` | Remove an entry by label |

## Add Options

| Short | Long | Default | Description |
|-------|------|---------|------------|
| `-q` | `--qr <path>` | | QR code image (PNG/JPEG), mutually exclusive with `-s` |
| `-l` | `--label <name>` | auto from QR | Entry label, used with `get` and `rm` |
| `-d` | `--domain <domain>` | | Service domain (e.g. github.com) |
| `-a` | `--account <account>` | | Account identifier (e.g. email) |
| `-s` | `--secret <base32>` | | Base32-encoded TOTP secret |
| `-A` | `--algorithm <alg>` | SHA1 | SHA1, SHA256, or SHA512 |
| `-n` | `--digits <n>` | 6 | Number of digits in the code |
| `-p` | `--period <secs>` | 30 | Time step in seconds |

## Global Flags

| Flag | Env Var | Default | Description |
|------|---------|---------|------------|
| `--config <path>` | `RUTH_STORE` | `~/.config/ruth-cli/store.toml.gpg` | Path to encrypted store |
| `--gpg-id <key>` | `RUTH_GPG_ID` | from `~/.config/ruth-cli/config.toml` | GPG recipient key ID |

## Config Resolution

Both the store path and GPG key ID follow the same precedence: **CLI flag > env var > config file**.

```
Store path:  --config  >  RUTH_STORE  >  ~/.config/ruth-cli/store.toml.gpg
GPG key ID:  --gpg-id  >  RUTH_GPG_ID  >  gpg_key_id in ~/.config/ruth-cli/config.toml
```

## Storage

Secrets are stored in `~/.config/ruth-cli/store.toml.gpg`, encrypted with your GPG key. The decrypted data is never written to disk — it is held in memory only and piped directly to/from `gpg`.

## Agent Integration

ruth-cli is designed to be used by AI agents and scripts:

- **Non-interactive**: all input via flags, env vars, or config files — no prompts
- **Clean stdout**: `get` outputs only the TOTP code, nothing else
- **Status on stderr**: informational messages (e.g. "Added entry...") go to stderr
- **Exit codes**: 0 on success, non-zero on error
- **Scriptable**: `TOKEN=$(ruth-cli get github)` captures the code directly

## Requirements

- [GPG](https://gnupg.org/) installed and a key pair configured
- A working `gpg-agent` for passphrase caching (recommended — avoids repeated passphrase prompts)
- Add `export GPG_TTY=$(tty)` to your shell profile for terminal pinentry support

## Development

Requires [just](https://github.com/casey/just) for task running:

```bash
just test       # Run all tests
just lint       # Run clippy
just check      # Run tests + clippy + fmt check
just release 0.1.0  # Tag and push a release (triggers GitHub Actions)
```

## Disclaimer

This project was co-developed with [Claude Code](https://claude.ai/claude-code) by Anthropic. While the TOTP implementation follows RFC 6238 and is tested against the official test vectors, this software is provided **as-is, without warranty of any kind**. Use it at your own risk.

The authors are not responsible for any loss of access to accounts, security breaches, or other damages resulting from the use of this tool. Always keep backup 2FA recovery codes for your accounts.
