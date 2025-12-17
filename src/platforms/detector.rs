use anyhow::{Result, bail};
use super::{atcoder::AtCoder, codeforces::Codeforces, Platform};

pub enum PlatformType {
    AtCoder(AtCoder),
    Codeforces(Codeforces),
}

impl PlatformType {
    pub fn name(&self) -> &str {
        match self {
            PlatformType::AtCoder(p) => p.name(),
            PlatformType::Codeforces(p) => p.name(),
        }
    }
}

pub fn detect_platform(url: &str) -> Result<PlatformType> {
    if AtCoder::detect(url) {
        Ok(PlatformType::AtCoder(AtCoder::new()))
    } else if Codeforces::detect(url) {
        Ok(PlatformType::Codeforces(Codeforces::new()))
    } else {
        bail!("Unsupported platform: {}", url)
    }
}
