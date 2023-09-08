use std::process::exit;
use ftp::FtpStream;

use crate::{
    config::SourcererConfig,
    commands::calculate
};

pub fn sync_cloud_to_local(config: &SourcererConfig) {
    println!("{config:#?}");
    todo!("sync_cloud_to_local");
}

pub fn sync_local_to_cloud(config: &SourcererConfig) {

    if config.ftp.is_none() {
        println!("No FTP configured");
        return;
    }

    let (files, total) = calculate(&config);

    let connection = match FtpStream::connect(
        format!("{}:{}", config.ftp.as_ref().unwrap_or_else(|| unreachable!()).host, config.ftp.as_ref().unwrap_or_else(|| unreachable!()).port)
    ) {
        Ok(c) => c,
        Err(e) => {
            println!("{}.\nExiting...", e);
            exit(0);
        }
    };

    println!("{config:#?}");
    todo!("sync_local_to_cloud");
}