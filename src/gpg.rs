use anyhow::{anyhow, Context, Result};
use std::process::{Command, Stdio};

pub fn check_gpg() -> Result<()> {
    Command::new("gpg")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map_err(|_| anyhow!(
            "gpg not found. Install GPG:\n  macOS: brew install gnupg\n  Linux: apt install gnupg"
        ))?;
    Ok(())
}

pub fn decrypt(path: &std::path::Path) -> Result<String> {
    let output = Command::new("gpg")
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
    let mut child = Command::new("gpg")
        .args([
            "--quiet",
            "--yes",
            "--batch",
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
