use anyhow::Result;
use super::traits::Storage;

pub struct S3Storage {
    bucket: String,
}

impl S3Storage {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self { bucket: bucket.into() }
    }
}

impl Storage for S3Storage {
    fn upload(&self, filename: &str, _data: &[u8]) -> Result<()> {
        println!("[S3] Uploading '{}' to bucket '{}'", filename, self.bucket);
        Ok(())
    }
}

pub struct SftpStorage {
    host: String,
}

impl SftpStorage {
    pub fn new(host: impl Into<String>) -> Self {
        Self { host: host.into() }
    }
}

impl Storage for SftpStorage {
    fn upload(&self, filename: &str, _data: &[u8]) -> Result<()> {
        println!("[SFTP] Uploading '{}' to '{}'", filename, self.host);
        Ok(())
    }
}
