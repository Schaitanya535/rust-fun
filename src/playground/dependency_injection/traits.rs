use anyhow::Result;

pub trait Storage {
    fn upload(&self, filename: &str, data: &[u8]) -> Result<()>;
}

pub trait Scanner {
    fn scan(&self, data: &[u8]) -> Result<bool>; // true = clean
}
