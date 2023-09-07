use serde::{
    Deserialize,
    Serialize
};
use std::{
    error::Error,
    fs::File,
    io::Read,
    fmt
};

#[derive(Deserialize, Serialize, Debug)]
pub struct SourcererConfig {
    pub include: Vec<String>
}

#[derive(Debug)]
pub enum ConfigError {
    FileNotFound,
    CannotRead,
    InvalidData
}

impl Error for ConfigError {}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::FileNotFound => write!(f, "File not found"),
            Self::CannotRead => write!(f, "Cannot read config file"),
            Self::InvalidData => write!(f, "Invalid data")
        }
    }
}

pub fn get_config() -> Result<SourcererConfig, ConfigError> {
    let mut file = match File::open("srccfg.json") {
        Ok(f) => f,
        Err(_) => {
            return Err(ConfigError::FileNotFound);
        }
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => {},
        Err(_) => {
            return Err(ConfigError::CannotRead);
        }
    };
    match serde_json::from_str(&contents) {
        Ok(c) => Ok(c),
        Err(_) => {
            return Err(ConfigError::InvalidData);
        }
    }
}