use std::env;
use std::fs;

use env_logger;
use log;

fn main() {
    // Setting logging
    env_logger::init();
    log::info!("Running my-git");

    let args: Vec<String> = env::args().collect();
    if args[1] == "init" {
        fs::create_dir(".git").unwrap();
        fs::create_dir(".git/objects").unwrap();
        fs::create_dir(".git/refs").unwrap();
        fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
        println!("Initialized git directory")
    } else {
        println!("unknown command: {}", args[1])
    }
}
