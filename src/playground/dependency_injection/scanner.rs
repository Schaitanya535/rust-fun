use anyhow::Result;
use super::traits::Scanner;

pub struct ThreatProtect;

impl Scanner for ThreatProtect {
    fn scan(&self, _data: &[u8]) -> Result<bool> {
        println!("[ThreatProtect] Scanning...");
        Ok(true)
    }
}

pub struct SynergyScanner;

impl Scanner for SynergyScanner {
    fn scan(&self, _data: &[u8]) -> Result<bool> {
        println!("[Synergy] Scanning with better detection...");
        Ok(true)
    }
}
