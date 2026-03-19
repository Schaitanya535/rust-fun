use super::storage::{S3Storage, SftpStorage};
use super::traits::Storage;

pub struct S4 {
    bucket: String,
}

pub enum CompanyStorage {
    S3 { bucket: String },
    Sftp { host: String },
    S5 { options: S4 },
    S4(S4),
}

impl CompanyStorage {
    pub fn into_storage(self) -> Box<dyn Storage> {
        match self {
            CompanyStorage::S3 { bucket } => Box::new(S3Storage::new(bucket)),
            CompanyStorage::Sftp { host } => Box::new(SftpStorage::new(host)),
            CompanyStorage::S4(S4 { bucket }) => Box::new(S3Storage::new(bucket)),
            CompanyStorage::S5 { options: S4 { bucket } } => Box::new(S3Storage::new(bucket)),
        }
    }
}
