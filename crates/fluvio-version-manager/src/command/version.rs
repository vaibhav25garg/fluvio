//! Prints version information to stdout

use anyhow::Result;
use clap::Args;
use current_platform::CURRENT_PLATFORM;
use sha2::{Digest, Sha256};
use sysinfo::SystemExt;

use crate::{BINARY_NAME, BINARY_VERSION};

#[derive(Debug, Args)]
pub struct VersionOpt;

impl VersionOpt {
    pub fn process(self) -> Result<()> {
        println!("{BINARY_NAME} CLI: {BINARY_VERSION}");
        println!("{BINARY_NAME} CLI Arch: {CURRENT_PLATFORM}");

        if let Some(sha) = self.format_cli_sha() {
            println!("{BINARY_NAME} CLI SHA256: {}", sha);
        }

        if let Some(info) = os_info() {
            println!("OS Details: {info}");
        }

        Ok(())
    }

    /// Read CLI and compute its sha256
    fn format_cli_sha(&self) -> Option<String> {
        let path = std::env::current_exe().ok()?;
        let bin = std::fs::read(path).ok()?;
        let mut hasher = Sha256::new();

        hasher.update(bin);

        let bin_sha256 = hasher.finalize();

        Some(format!("{:x}", &bin_sha256))
    }
}

fn os_info() -> Option<String> {
    let sys = sysinfo::System::new_all();
    let info = format!(
        "{} {} (kernel {})",
        sys.name()?,
        sys.os_version()?,
        sys.kernel_version()?,
    );

    Some(info)
}
