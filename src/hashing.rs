use data_encoding::HEXUPPER;
use glob::glob;
use ring::digest::{
    Context,
    Digest,
    SHA256
};
use std::{
    collections::BTreeMap,
    fs::File,
    io::{
        Read,
        BufReader
    }, 
};
use walkdir::WalkDir;


pub fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest, ()> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024*16];

    loop {
        let count = reader.read(&mut buffer).unwrap_or_else(|_| panic!("Cannot read file"));
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}

pub fn calculate_hashes(include: Vec<String>) -> BTreeMap<String, String> {
    let mut data = BTreeMap::new();

    for item in include {
        for entry in glob(&item).expect("Invalid glob pattern") {
            let entry = match entry {
                Ok(e) => e,
                Err(e) => {
                    println!("File {} found, but cannot be openned", e.path().display());
                    continue;
                }
            };
            if entry.is_file() {
                if data.contains_key(&entry.to_str().unwrap().to_string()) {
                    continue;
                }
                let input = File::open(&entry).unwrap();
                let reader = BufReader::new(input);
                let digest = sha256_digest(reader).unwrap();
                data.insert(entry.to_str().unwrap().to_string(), HEXUPPER.encode(digest.as_ref()));
            } else {
                for item in WalkDir::new(entry)
                .into_iter()
                .filter_map(Result::ok)
                .filter(|e| !e.file_type().is_dir()) {
                    if data.contains_key(&item.path().display().to_string()) {
                        continue;
                    }
                    let input = File::open(item.path()).unwrap();
                    let reader = BufReader::new(input);
                    let digest = sha256_digest(reader).unwrap();
                    data.insert(item.path().display().to_string(), HEXUPPER.encode(digest.as_ref()));
                }
            }
        }
    }
    data
}