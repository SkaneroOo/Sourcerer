use std::{
    path::Path, 
    fs::File,
    io::Write,
    env
};

use crate::config::SourcererConfig;

pub fn init() {
    let path = Path::new("srccfg.json");
    println!("{}", env::current_dir().unwrap().file_name().unwrap().to_str().unwrap());
    if path.exists() {
        println!("Config file for this project already exists");
        return;
    }
    let config = SourcererConfig{
        name: match env::current_dir() {
            Ok(p) => match p.file_name() {
                Some(f) => match f.to_str() {
                    Some(f) => f.to_string(),
                    None => panic!()
                },
                None => panic!()
            },
            Err(_) => panic!()
        },
        ..Default::default()
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