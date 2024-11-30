mod commands;
use commands::{
    hash_object, initialize_git_repository, list_tree_contents, print_git_object_contents,
    write_tree,
};

use std::env;

use env_logger;
use log;

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
        "hash-object" => {
            hash_object(&args[2..]);
        } // Hash object
        "ls-tree" => list_tree_contents(&args[2..]), // List tree contents
        "write-tree" => {
            write_tree(&args[2..]);
        } // Write tree
        _ => println!("unknown command: {}", args[1]),
    }
}
