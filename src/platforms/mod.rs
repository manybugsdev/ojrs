pub mod atcoder;
pub mod codeforces;
pub mod detector;

use anyhow::Result;

#[derive(Debug, Clone)]
pub struct TestCase {
    pub input: String,
    pub output: String,
}

pub trait Platform {
    fn detect(url: &str) -> bool where Self: Sized;
    fn name(&self) -> &str;
}

pub trait Downloadable {
    async fn download_test_cases(&self, url: &str) -> Result<Vec<TestCase>>;
}

pub trait Submittable {
    async fn submit(&self, url: &str, code: &str) -> Result<String>;
}

pub trait Loginable {
    async fn login(&self, url: &str) -> Result<()>;
}
