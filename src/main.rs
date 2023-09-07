use std::{
    process::exit,
    env
};

mod config;
mod hashing;
mod commands;

use commands::{init, add, sync_cloud_to_local, sync_local_to_cloud};
use config::get_config;


fn main() {

    let mut args = env::args();
    let exec = args.next().unwrap_or_else(|| unreachable!());

    let Some(command) = args.next() else {
        println!("Usage: {} <command>", exec);
        println!("Available commands:");
        println!("  calculate - Calculate hashes of all selected files");
        println!("  init - Initialize config file");
        println!("  add - Add files to config file");
        println!("  syncctl - Sync cloud files to local");
        println!("  syncltc - Sync local files to cloud");
        exit(0)
    };


    match command.as_str() {
        "calculate" => {
            let config = match get_config() {
                Ok(c) => c,
                Err(e) => {
                    println!("{}.\nExiting...", e);
                    exit(0);
                }
            };
            let (files, project) = commands::calculate(config);
            println!("File hashes:\n{:#?}", files);
            println!("Project hash: {}", project);
        },
        "init" => {
            init();
        },
        "add" => {
            let config = match get_config() {
                Ok(c) => c,
                Err(e) => {
                    println!("{}.\nExiting...", e);
                    exit(0);
                }
            };
            add(config, args.collect());
        },
        "syncctl" => {
            let config = match get_config() {
                Ok(c) => c,
                Err(e) => {
                    println!("{}.\nExiting...", e);
                    exit(0);
                }
            };
            sync_cloud_to_local(config);
        }
        "syncltc" => {
            let config = match get_config() {
                Ok(c) => c,
                Err(e) => {
                    println!("{}.\nExiting...", e);
                    exit(0);
                }
            };
            sync_local_to_cloud(config);
        }
        _ => {
            println!("Unknown command `{}`.\nExiting...", command);
        }
    }
}
