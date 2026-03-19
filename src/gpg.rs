use anyhow::{anyhow, Context, Result};
use std::process::{Command, Stdio};

/// Build a base GPG command with GPG_TTY set for pinentry support.
fn gpg_command() -> Command {
    let mut cmd = Command::new("gpg");
    // Ensure GPG can find the terminal for passphrase prompts
    if let Ok(tty) = std::env::var("GPG_TTY") {
        cmd.env("GPG_TTY", tty);
    } else if let Ok(tty_path) = std::fs::read_link("/dev/fd/0")
        .or_else(|_| std::env::var("TTY").map(std::path::PathBuf::from))
    {
        cmd.env("GPG_TTY", tty_path);
    }
    cmd
}

pub fn check_gpg() -> Result<()> {
    Command::new("gpg")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map_err(|_| {
            anyhow!(
                "gpg not found. Install GPG:\n  macOS: brew install gnupg\n  Linux: apt install gnupg"
            )
        })?;
    Ok(())
}

pub fn decrypt(path: &std::path::Path) -> Result<String> {
    let output = gpg_command()
        .args(["--quiet", "--yes", "--batch", "--decrypt"])
        .arg(path)
        .output()
        .context("failed to run gpg decrypt")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("gpg decrypt failed: {}", stderr.trim()));
    }

    String::from_utf8(output.stdout).context("gpg output is not valid UTF-8")
}

pub fn encrypt(plaintext: &str, recipient: &str, path: &std::path::Path) -> Result<()> {
    let mut child = gpg_command()
        .args([
            "--quiet",
            "--yes",
            "--batch",
            "--trust-model",
            "always",
            "--encrypt",
            "--recipient",
            recipient,
            "--output",
        ])
        .arg(path)
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("failed to run gpg encrypt")?;

    use std::io::Write;
    child
        .stdin
        .take()
        .unwrap()
        .write_all(plaintext.as_bytes())
        .context("failed to write to gpg stdin")?;

    let output = child.wait_with_output().context("gpg encrypt failed")?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("gpg encrypt failed: {}", stderr.trim()));
    }

    Ok(())
}
