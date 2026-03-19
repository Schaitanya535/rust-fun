use anyhow::Result;
use super::traits::{Storage, Scanner};

pub struct AttachmentService {
    storage: Box<dyn Storage>,
    scanner: Box<dyn Scanner>,
}

impl AttachmentService {
    pub fn new(storage: Box<dyn Storage>, scanner: Box<dyn Scanner>) -> Self {
        Self { storage, scanner }
    }

    pub fn upload(&self, filename: &str, data: &[u8]) -> Result<()> {
        let clean = self.scanner.scan(data)?;
        if !clean {
            anyhow::bail!("File '{}' failed virus scan", filename);
        }
        self.storage.upload(filename, data)?;
        Ok(())
    }
}
