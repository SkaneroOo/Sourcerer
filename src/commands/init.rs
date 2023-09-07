use std::{
    path::Path, 
    fs::File,
    io::Write
};

use crate::config::SourcererConfig;

pub fn init() {
    let path = Path::new("srccfg.json");
    if path.exists() {
        println!("Config file for this project already exists");
        return;
    }
    let config = SourcererConfig{
        include: vec![]
    };
    let mut file = match File::create(path) {
        Ok(f) => f,
        Err(_) => {
            println!("Cannot create config file\nExiting...");
            return;
        }
    };
    write!(file, "{}", serde_json::to_string_pretty(&config).unwrap_or(r#"{
    "include": []
}"#.to_string())).expect("Cannot write config file");
    println!("Successfully created config file");
}