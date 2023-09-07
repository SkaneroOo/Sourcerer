use std::collections::BTreeMap;

use data_encoding::HEXUPPER;

use crate::{
    hashing::{
        calculate_hashes, 
        sha256_digest
    }, 
    config::SourcererConfig
};

pub fn calculate(config: SourcererConfig) -> (BTreeMap<String, String>, String) {
    let data = calculate_hashes(config.include);

    let digest = sha256_digest(serde_json::to_string(&data).unwrap_or_default().as_bytes()).unwrap();

    (data, HEXUPPER.encode(digest.as_ref()))
}