use std::env;
use std::fs;
use std::io::prelude::*;

use env_logger;
use flate2::read::ZlibDecoder;
use log;

fn initialize_git_repository() {
    fs::create_dir(".git").expect("Failed to create .git directory");
    fs::create_dir(".git/objects").expect("Failed to create .git/objects directory");
    fs::create_dir(".git/refs").expect("Failed to create .git/refs directory");
    fs::write(".git/HEAD", "ref: refs/heads/main\n").expect("Failed to create .git/HEAD file");
}

fn print_git_object_contents(args: &[String]) {
    if args.len() != 2 {
        println!("usage: my-git cat-file -t <hash>");
        println!("usage: my-git cat-file -p <hash>");
        std::process::exit(1);
    }

    let _obj_type = &args[0];
    let hash = &args[1];

    let content: Vec<u8> = fs::read(format!(".git/objects/{}/{}", &hash[..2], &hash[2..]))
        .expect("Failed to read object");

    let mut z_lib_decoder = ZlibDecoder::new(&content[..]);
    let mut buffer = String::new();
    z_lib_decoder
        .read_to_string(&mut buffer)
        .expect("Failed to read object content");

    print!("{}", &buffer[8..]);
}

fn main() {
    // Setting logging
    env_logger::init();
    log::info!("Running my-git");

    // Collecting arguments
    let args: Vec<String> = env::args().collect();

    // Commands
    match args[1].as_str() {
        "init" => initialize_git_repository(), // Initialize git repository
        "cat-file" => print_git_object_contents(&args[2..]), // Print git object contents
        _ => println!("unknown command: {}", args[1]),
    }
}
