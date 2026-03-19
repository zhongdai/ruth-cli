# GPG Setup Guide

This guide covers installing GPG, creating a key pair, and finding your key ID for use with `ruth-cli`.

## Install GPG

### macOS

```bash
brew install gnupg
```

### Ubuntu / Debian

```bash
sudo apt update && sudo apt install -y gnupg
```

### Fedora / RHEL

```bash
sudo dnf install gnupg2
```

### Arch Linux

```bash
sudo pacman -S gnupg
```

### Verify installation

```bash
gpg --version
```

## Create a Key Pair

```bash
gpg --full-generate-key
```

You will be prompted for:

1. **Key type** — select `(1) RSA and RSA` (default)
2. **Key size** — `4096` recommended
3. **Expiration** — `0` for no expiration, or set a duration (e.g., `1y`)
4. **Real name** — your name
5. **Email** — your email address
6. **Passphrase** — a strong passphrase to protect the key

Example session:

```
Please select what kind of key you want:
   (1) RSA and RSA
Your selection? 1

What keysize do you want? 4096

Key is valid for? 0

Real name: Jane Doe
Email address: jane@example.com
Comment:
You selected this USER-ID:
    "Jane Doe <jane@example.com>"

Change (N)ame, (C)omment, (E)mail or (O)kay/(Q)uit? O
```

## Find Your Key ID

### List keys

```bash
gpg --list-keys --keyid-format long
```

Output looks like:

```
pub   rsa4096/ABCDEF1234567890 2024-01-15 [SC]
      1234567890ABCDEF1234567890ABCDEF12345678
uid                 [ultimate] Jane Doe <jane@example.com>
sub   rsa4096/0987654321FEDCBA 2024-01-15 [E]
```

### What to use as your key ID

Any of these formats work with `ruth-cli`:

| Format | Example | Recommended |
|--------|---------|-------------|
| Email | `jane@example.com` | Yes — easiest to remember |
| Long key ID | `ABCDEF1234567890` | Yes — unambiguous |
| Fingerprint | `1234567890ABCDEF...` | Yes — most specific |
| Short key ID | `12345678` | No — collision risk |

### Configure ruth-cli with your key

```bash
# Option A: config file (recommended)
mkdir -p ~/.config/ruth-cli
echo 'gpg_key_id = "jane@example.com"' > ~/.config/ruth-cli/config.toml

# Option B: environment variable
export RUTH_GPG_ID="jane@example.com"

# Option C: per-command flag
ruth-cli add --gpg-id jane@example.com --label myservice --secret JBSWY3DPEHPK3PXP --domain example.com --account me@example.com
```

## GPG Agent (Passphrase Caching)

To avoid typing your passphrase every time, ensure `gpg-agent` is running:

```bash
# Check if agent is running
gpg-connect-agent /bye

# If not running, start it
gpgconf --launch gpg-agent
```

Configure cache duration in `~/.gnupg/gpg-agent.conf`:

```
# Cache passphrase for 8 hours (28800 seconds)
default-cache-ttl 28800
max-cache-ttl 28800
```

Reload after changes:

```bash
gpg-connect-agent reloadagent /bye
```

## Backup Your Key

**Export your private key** (store this securely):

```bash
gpg --export-secret-keys --armor jane@example.com > private-key.asc
```

**Export your public key**:

```bash
gpg --export --armor jane@example.com > public-key.asc
```

**Import on another machine**:

```bash
gpg --import private-key.asc
gpg --edit-key jane@example.com trust  # set trust level to 5 (ultimate)
```

## Troubleshooting

### "gpg: decryption failed: No secret key"

Your private key is not available. Import it or check `gpg --list-secret-keys`.

### "gpg: public key not found"

The key ID in your ruth-cli config doesn't match any key in your keyring. Run `gpg --list-keys` to verify.

### "gpg: signing failed: Inappropriate ioctl for device"

GPG can't open a pinentry dialog. Fix:

```bash
export GPG_TTY=$(tty)
```

Add this to your `~/.bashrc` or `~/.zshrc` to make it permanent.
