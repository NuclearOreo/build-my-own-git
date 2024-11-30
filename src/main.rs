use std::env;
use std::fs;
use std::io::prelude::*;

use env_logger;
use flate2::read::ZlibDecoder;
use log;

fn main() {
    // Setting logging
    env_logger::init();
    log::info!("Running my-git");

    // Collecting arguments
    let args: Vec<String> = env::args().collect();

    // Commands
    if args[1] == "init" {
        // Initialize git repository
        fs::create_dir(".git").expect("Failed to create .git directory");
        fs::create_dir(".git/objects").expect("Failed to create .git/objects directory");
        fs::create_dir(".git/refs").expect("Failed to create .git/refs directory");
        fs::write(".git/HEAD", "ref: refs/heads/main\n").expect("Failed to create .git/HEAD file");
        println!("Initialized git directory")
    } else if args[1] == "cat-file" {
        // Print the contents of a git object
        // Flags: -p, -s, -t
        let _flag = &args[2];
        let hash = &args[3];

        let content = fs::read(format!(".git/objects/{}/{}", &hash[..2], &hash[2..]))
            .expect("Failed to read object");

        let mut z_lib_decoder = ZlibDecoder::new(&content[..]);
        let mut buffer = String::new();
        z_lib_decoder
            .read_to_string(&mut buffer)
            .expect("Failed to read object content");

        print!("{}", &buffer[8..]);
    } else {
        println!("unknown command: {}", args[1])
    }
}
