use anyhow::{anyhow, Context, Result};
use url::Url;

#[derive(Debug)]
pub struct OtpAuthData {
    pub secret: String,
    pub issuer: Option<String>,
    pub account: Option<String>,
    pub algorithm: String,
    pub digits: u32,
    pub period: u64,
}

impl OtpAuthData {
    pub fn auto_label(&self) -> String {
        match (&self.issuer, &self.account) {
            (Some(issuer), Some(account)) => format!("{}-{}", issuer, account),
            (Some(issuer), None) => issuer.clone(),
            (None, Some(account)) => account.clone(),
            (None, None) => String::new(),
        }
    }
}

pub fn parse_otpauth_uri(uri: &str) -> Result<OtpAuthData> {
    let url = Url::parse(uri).context("invalid otpauth URI")?;

    if url.scheme() != "otpauth" {
        return Err(anyhow!("not an otpauth URI"));
    }
    if url.host_str() != Some("totp") {
        return Err(anyhow!("only TOTP is supported, got: {:?}", url.host_str()));
    }

    let path = url.path().trim_start_matches('/');
    let (issuer_from_path, account_from_path) = if path.contains(':') {
        let mut parts = path.splitn(2, ':');
        (
            Some(parts.next().unwrap().to_string()),
            Some(parts.next().unwrap().to_string()),
        )
    } else if !path.is_empty() {
        // Path without ':' could be issuer or account.
        // If it matches the issuer query param, treat as issuer label (not account).
        // If it contains '@', treat as account. Otherwise treat as issuer.
        if path.contains('@') {
            (None, Some(path.to_string()))
        } else {
            (Some(path.to_string()), None)
        }
    } else {
        (None, None)
    };

    let params: std::collections::HashMap<_, _> = url.query_pairs().collect();

    let secret = params
        .get("secret")
        .ok_or_else(|| anyhow!("missing required 'secret' parameter"))?
        .to_string();

    let issuer = params
        .get("issuer")
        .map(|s| s.to_string())
        .or(issuer_from_path);

    let account = account_from_path;

    let algorithm = params
        .get("algorithm")
        .map(|s| s.to_uppercase())
        .unwrap_or_else(|| "SHA1".to_string());

    let digits = params
        .get("digits")
        .map(|s| s.parse::<u32>())
        .transpose()
        .context("invalid digits")?
        .unwrap_or(6);

    let period = params
        .get("period")
        .map(|s| s.parse::<u64>())
        .transpose()
        .context("invalid period")?
        .unwrap_or(30);

    Ok(OtpAuthData {
        secret,
        issuer,
        account,
        algorithm,
        digits,
        period,
    })
}

pub fn decode_qr_image(path: &std::path::Path) -> Result<String> {
    let img = image::open(path).context("failed to open image")?;
    let gray = img.to_luma8();

    let mut prepared = rqrr::PreparedImage::prepare(gray);
    let grids = prepared.detect_grids();

    if grids.is_empty() {
        return Err(anyhow!("no QR code found in image"));
    }

    let (_meta, content) = grids[0].decode().context("failed to decode QR code")?;
    Ok(content)
}

pub fn from_qr_image(path: &std::path::Path) -> Result<OtpAuthData> {
    let uri = decode_qr_image(path)?;
    parse_otpauth_uri(&uri)
}
