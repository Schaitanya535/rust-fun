pub mod factory;
pub mod scanner;
pub mod service;
pub mod storage;
pub mod traits;

use anyhow::Result;

pub fn run() -> Result<()> {
    let is_dev = true;

    let scanner: Box<dyn traits::Scanner> = if is_dev {
        Box::new(scanner::SynergyScanner)
    } else {
        Box::new(scanner::ThreatProtect)
    };

    let storage = factory::CompanyStorage::S3 {
        bucket: "acme-attachments".into(),
    }.into_storage();

    let service = service::AttachmentService::new(storage, scanner);
    service.upload("report.pdf", b"fake file content")?;

    println!("---");

    let storage2 = factory::CompanyStorage::Sftp {
        host: "sftp.picky-client.com".into(),
    }.into_storage();
    let scanner2: Box<dyn traits::Scanner> = Box::new(scanner::ThreatProtect);

    let service2 = service::AttachmentService::new(storage2, scanner2);
    service2.upload("contract.pdf", b"fake file content")?;

    Ok(())
}
