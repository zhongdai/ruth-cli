use anyhow::{anyhow, Result};
use data_encoding::BASE32;
use hmac::{Hmac, Mac};
use sha1::Sha1;
use sha2::{Sha256, Sha512};

#[derive(Debug, Clone, Copy, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Algorithm {
    #[default]
    SHA1,
    SHA256,
    SHA512,
}

impl std::fmt::Display for Algorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Algorithm::SHA1 => write!(f, "SHA1"),
            Algorithm::SHA256 => write!(f, "SHA256"),
            Algorithm::SHA512 => write!(f, "SHA512"),
        }
    }
}

impl std::str::FromStr for Algorithm {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s.to_uppercase().as_str() {
            "SHA1" => Ok(Algorithm::SHA1),
            "SHA256" => Ok(Algorithm::SHA256),
            "SHA512" => Ok(Algorithm::SHA512),
            other => Err(anyhow!("unsupported algorithm: {}", other)),
        }
    }
}

fn normalize_base32(secret: &str) -> String {
    let clean: String = secret
        .chars()
        .filter(|c| !c.is_whitespace() && *c != '=')
        .collect::<String>()
        .to_uppercase();
    let pad_len = (8 - clean.len() % 8) % 8;
    let mut padded = clean;
    padded.extend(std::iter::repeat('=').take(pad_len));
    padded
}

pub fn validate_secret(secret: &str) -> Result<()> {
    let padded = normalize_base32(secret);
    BASE32
        .decode(padded.as_bytes())
        .map_err(|e| anyhow!("invalid base32 secret: {}", e))?;
    Ok(())
}

pub fn generate(
    secret: &str,
    time: u64,
    period: u64,
    digits: u32,
    algorithm: Algorithm,
) -> Result<String> {
    let padded = normalize_base32(secret);
    let key = BASE32
        .decode(padded.as_bytes())
        .map_err(|e| anyhow!("invalid base32 secret: {}", e))?;

    let counter = time / period;
    let counter_bytes = counter.to_be_bytes();

    let hmac_result = match algorithm {
        Algorithm::SHA1 => {
            let mut mac =
                Hmac::<Sha1>::new_from_slice(&key).map_err(|e| anyhow!("HMAC key error: {}", e))?;
            mac.update(&counter_bytes);
            mac.finalize().into_bytes().to_vec()
        }
        Algorithm::SHA256 => {
            let mut mac = Hmac::<Sha256>::new_from_slice(&key)
                .map_err(|e| anyhow!("HMAC key error: {}", e))?;
            mac.update(&counter_bytes);
            mac.finalize().into_bytes().to_vec()
        }
        Algorithm::SHA512 => {
            let mut mac = Hmac::<Sha512>::new_from_slice(&key)
                .map_err(|e| anyhow!("HMAC key error: {}", e))?;
            mac.update(&counter_bytes);
            mac.finalize().into_bytes().to_vec()
        }
    };

    let offset = (hmac_result[hmac_result.len() - 1] & 0x0f) as usize;
    let code = u32::from_be_bytes([
        hmac_result[offset] & 0x7f,
        hmac_result[offset + 1],
        hmac_result[offset + 2],
        hmac_result[offset + 3],
    ]);

    let modulus = 10u32.pow(digits);
    Ok(format!(
        "{:0>width$}",
        code % modulus,
        width = digits as usize
    ))
}

pub fn generate_now(
    secret: &str,
    period: u64,
    digits: u32,
    algorithm: Algorithm,
) -> Result<String> {
    let time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| anyhow!("system time error: {}", e))?
        .as_secs();
    generate(secret, time, period, digits, algorithm)
}
