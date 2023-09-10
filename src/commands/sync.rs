use std::{process::exit, collections::BTreeMap, io::{Cursor, BufReader}, fs::File, path::Path};
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

    let (_files, _total) = calculate(&config);

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
    
    let _hashes = connection.retr("hashes.json", |stream| {
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

fn recursive_cloud_mkdir(connection: &mut FtpStream, path: &Path) {
    let parent = path.parent();
    if let Some(parent) = parent {
        recursive_cloud_mkdir(connection, parent);
    }
    let path = path.to_str().unwrap();
    if path.is_empty() {
        return;
    }
    connection.mkdir(path).unwrap_or_else(|_| {
        return;
    })
}

pub fn sync_local_to_cloud(config: &SourcererConfig) {

    if config.ftp.is_none() {
        println!("No FTP configured");
        return;
    }

    let ftp_config = config.ftp.as_ref().unwrap_or_else(|| unreachable!());

    let (mut files, _) = calculate(&config);

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
        connection.cwd(&config.name).unwrap_or_else(|_| {
            panic!("Cannot enter just created directory");
        })
    });
    
    let mut cursor = Cursor::new(serde_json::to_string(&files).unwrap_or_default());
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
            for (key, hash) in hashes {
                if let Some(local_hash) = files.get(&key) {
                    if hash.eq(local_hash) {
                        files.remove(&key);
                    }
                }
            }
        },
        Err(_) => {}
    }
    connection.put("hashes.json", &mut cursor).unwrap_or_else(|e| {
        println!("Cannot write hashes file: {}.\nExiting...", e);
    });
    for file in files.keys() {
        println!("Wrining file: {}", file);
        let path = Path::new(file);
        if let Some(parrent) = path.parent() {
            recursive_cloud_mkdir(&mut connection, parrent);
        }
        let input = File::open(&file).unwrap();
        let mut reader = BufReader::new(input);
        connection.put(&file, &mut reader).unwrap_or_else(|e| {
            println!("Cannot write file: {}.\nExiting...", e);
            exit(0);
        })
    }
    

}