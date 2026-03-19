# ruth-cli Design Spec

**Date:** 2026-03-19
**Status:** Draft

## Overview

`ruth-cli` is an agent-friendly, non-interactive CLI tool for generating TOTP (Time-based One-Time Password) codes per RFC 6238. It stores secrets in a GPG-encrypted TOML file and requires no interactive input, making it suitable for automation and AI agent workflows.

## CLI Interface

```
ruth-cli add --qr /path/to/qr.png [--label <label>]
ruth-cli add --label <label> --domain <domain> --account <account> --secret <secret> [--digits 6] [--period 30]
ruth-cli get <label>
ruth-cli list
ruth-cli rm <label>
```

### Subcommands

- **`add --qr`** — Reads a QR code image, parses the `otpauth://` URI, and auto-populates all fields. Label defaults to `issuer-account` if not provided. Supported image formats: PNG, JPEG.
- **`add --label`** — Manual entry fallback when a raw secret is available but no QR image. `--qr` and `--label --secret` are mutually exclusive.
- **`get <label>`** — Decrypts the store, computes the current TOTP code, prints only the 6-digit (or configured digits) code to stdout. Exit code 0 on success, non-zero on error.
- **`list`** — Decrypts the store, prints all labels one per line to stdout.
- **`rm <label>`** — Removes the entry matching the label.

### Global Flags

- `--config <path>` — Override the store file location.
- `--gpg-id <key_id>` — Override the GPG recipient key.

## Data Model & Storage

### Encrypted Store

Default location: `~/.config/ruth-cli/store.toml.gpg`

```toml
[[entry]]
label = "github-work"
domain = "github.com"
account = "user@work.com"
secret = "JBSWY3DPEHPK3PXP"
digits = 6
period = 30

[[entry]]
label = "aws-prod"
domain = "aws.amazon.com"
account = "admin@company.com"
secret = "KRSXG5CTMVRXEZLU"
digits = 6
period = 30
```

### Plaintext Config

Location: `~/.config/ruth-cli/config.toml`

```toml
gpg_key_id = "user@example.com"
```

### Config Resolution

**Store path (first match wins):**
1. `--config /path/to/store.toml.gpg` (CLI flag)
2. `RUTH_STORE` env var
3. `~/.config/ruth-cli/store.toml.gpg` (default)

**GPG key ID (first match wins):**
1. `--gpg-id user@example.com` (CLI flag)
2. `RUTH_GPG_ID` env var
3. `gpg_key_id` from `~/.config/ruth-cli/config.toml`

### Mutation Workflow (add/rm)

1. Decrypt store to TOML in memory
2. Modify entries
3. Serialize to TOML, re-encrypt, write back to store path

## TOTP Algorithm (RFC 6238)

Implementation of TOTP on top of HOTP (RFC 4226):

1. **Decode** the base32-encoded secret into bytes
2. **Compute time step:** `T = floor(current_unix_time / period)`
3. **HMAC-SHA1:** `hmac_sha1(secret_bytes, T as big-endian u64)`
4. **Dynamic truncation:** Extract 4 bytes from the HMAC using the offset in the last nibble
5. **Modulo:** `truncated_value % 10^digits`, zero-padded to `digits` length

**Crates used:** `hmac` + `sha1` for HMAC-SHA1, `data-encoding` for base32 decoding. No external TOTP library — the algorithm is ~30 lines.

## GPG Integration

Shells out to the `gpg` CLI (same approach as `pass`).

**Decrypt:**
```
gpg --quiet --yes --batch --decrypt store.toml.gpg
```
Ciphertext read from file, plaintext TOML captured from stdout.

**Encrypt:**
```
gpg --quiet --yes --batch --encrypt --recipient <gpg_key_id> --output store.toml.gpg
```
Plaintext TOML piped to stdin.

**Init flow (first `add` when no store exists):**
1. Verify GPG key ID is configured — error if not
2. Create empty TOML, encrypt, write to store path

## QR Code Support

**Crates:** `image` + `rqrr` for QR decoding, `url` for URI parsing.

**Flow:**
1. Read image file (PNG/JPEG)
2. Decode QR code to string
3. Parse `otpauth://totp/Label?secret=...&issuer=...&digits=...&period=...`
4. Extract fields: secret (required), issuer, account, digits, period
5. Auto-generate label: `issuer-account` > `issuer` > `account` > require `--label`

## Project Structure

```
ruth-cli/
├── Cargo.toml
├── src/
│   ├── main.rs          # CLI entry point, clap setup
│   ├── cli.rs           # Clap derive structs
│   ├── store.rs         # TOML (de)serialization, entry CRUD
│   ├── gpg.rs           # Shell out to gpg, encrypt/decrypt
│   ├── totp.rs          # RFC 6238 implementation
│   ├── qr.rs            # QR decode + otpauth:// URI parsing
│   └── config.rs        # Config resolution (flag > env > file)
├── README.md
└── .github/
    └── workflows/
        └── release.yml  # Build + publish artifacts
```

## Dependencies

- `clap` (derive) — CLI argument parsing
- `serde` + `toml` — TOML serialization/deserialization
- `hmac` + `sha1` — TOTP HMAC computation
- `data-encoding` — base32 decoding
- `image` + `rqrr` — QR code image reading and decoding
- `url` — parsing `otpauth://` URIs

## GitHub Actions

**`release.yml`** — triggered on tag push (e.g., `v0.1.0`):
- Matrix build: `x86_64-linux`, `aarch64-linux`, `x86_64-macos`, `aarch64-macos`
- Upload compiled binaries as GitHub release artifacts

## Error Handling

- **Duplicate labels:** `add` rejects if label already exists. User must `rm` first.
- **Missing QR fields:** Falls back to available fields. Secret is required; if missing, error. Label auto-generation falls through: `issuer-account` > `issuer` > `account` > require `--label`.
- **GPG not found:** Clear error with install hint.
- **Decryption failure:** Surface `gpg` stderr to user.
- **No store on get/list/rm:** "No store found. Run `ruth-cli add` first."
- **Invalid secret on get:** "Invalid secret for label X, re-add the entry."
- **Time sync:** TOTP depends on system clock. No NTP check — clock issues are outside tool scope.
- **Concurrent access:** Not handled. Single-user CLI tool; OS-level file locking from GPG surfaces naturally.
