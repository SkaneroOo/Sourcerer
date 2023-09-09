use std::{process::exit, collections::BTreeMap};
use ftp::FtpStream;

use crate::{
    config::SourcererConfig,
    commands::calculate
};

pub fn sync_cloud_to_local(config: &SourcererConfig) {
    if config.ftp.is_none() {
        println!("No FTP configured");
        return;
    }

    let ftp_config = config.ftp.as_ref().unwrap_or_else(|| unreachable!());

    let (files, total) = calculate(&config);

    let mut connection = match FtpStream::connect(
        format!("{}:{}", ftp_config.host, ftp_config.port)
    ) {
        Ok(c) => c,
        Err(e) => {
            println!("{}.\nExiting...", e);
            exit(0);
        }
    };

    connection.login(
        &ftp_config.user, 
        &ftp_config.password
    ).unwrap_or_else(|e| {
        println!("{}.\nExiting...", e);
        exit(0);
    });

    connection.cwd(&ftp_config.dir).unwrap_or_else(|_| {
        println!("Directory {} not found on server", ftp_config.dir);
        exit(0);
    });

    connection.cwd(&config.name).unwrap_or_else(|_| {
        println!("Directory {} not found on server\nExiting...", config.name);
        exit(0);
    });
    
    let hashes = connection.retr("hashes.json", |stream| {
        let mut contents = String::new();
        stream.read_to_string(&mut contents).unwrap_or_else(|e| {
            println!("{}.\nExiting...", e);
            exit(0);
        });
        let hashes: BTreeMap<String, String> = serde_json::from_str(&contents).unwrap_or_else(|e| {
            println!("{}.\nExiting...", e);
            exit(0);
        });
        Ok(hashes)
    }).unwrap_or_else(|_| {
        println!("Project data file doesn't exist on server\nExiting...");
        exit(0);
    });

    todo!("sync local project with cloud files")
}

pub fn sync_local_to_cloud(config: &SourcererConfig) {

    if config.ftp.is_none() {
        println!("No FTP configured");
        return;
    }

    let ftp_config = config.ftp.as_ref().unwrap_or_else(|| unreachable!());

    let (files, total) = calculate(&config);

    let mut connection = match FtpStream::connect(
        format!("{}:{}", ftp_config.host, ftp_config.port)
    ) {
        Ok(c) => c,
        Err(e) => {
            println!("{}.\nExiting...", e);
            exit(0);
        }
    };

    connection.login(
        &ftp_config.user, 
        &ftp_config.password
    ).unwrap_or_else(|e| {
        println!("{}.\nExiting...", e);
        exit(0);
    });

    connection.cwd(&ftp_config.dir).unwrap_or_else(|_| {
        println!("Directory {} not found on server", ftp_config.dir);
        exit(0);
    });

    connection.cwd(&config.name).unwrap_or_else(|_| {
        connection.mkdir(&config.name).unwrap_or_else(|e| {
            println!("Cannot create directory {}: {}", config.name, e);
            exit(0);
        });
    });
    
    match connection.retr("hashes.json", |stream| {
        let mut contents = String::new();
        stream.read_to_string(&mut contents).unwrap_or_else(|e| {
            println!("{}.\nExiting...", e);
            exit(0);
        });
        let hashes: BTreeMap<String, String> = serde_json::from_str(&contents).unwrap_or_else(|e| {
            println!("{}.\nExiting...", e);
            exit(0);
        });
        Ok(hashes)
    }) {
        Ok(hashes) => {
            todo!("sync project with exising cloud files")
        },
        Err(e) => {
            todo!("create new cloud project")
        }
    }
    

}