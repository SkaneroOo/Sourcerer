use std::{
    path::Path, 
    fs::File,
    io::Write
};

use crate::config::SourcererConfig;

pub fn add(config: &SourcererConfig, params: Vec<String>) {
    if params.is_empty() {
        println!("No files provided");
        return;
    }
    let mut new = SourcererConfig {
        include: config.include.clone(),
        ftp: match &config.ftp {
            Some(ftp) => Some(ftp.clone()),
            None => None
        },
        name: config.name.clone(),
    };
    for param in &params {
        if !new.include.contains(&param) {
            new.include.push(param.to_string());
        }
    }

    let path = Path::new("srccfg.json");
    let mut file = match File::create(path) {
        Ok(f) => f,
        Err(_) => {
            println!("Cannot open config file\nExiting...");
            return;
        }
    };
    write!(file, "{}", serde_json::to_string_pretty(&new).unwrap_or_else(|_| unreachable!())).expect("Cannot write config file");
    println!("Successfully included {} in config file", params.join(", "));
}