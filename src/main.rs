mod commands;
mod utils;
use commands::{
    clone_repository, commit_tree, hash_object, initialize_git_repository, list_tree_contents,
    print_git_object_contents, write_tree,
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
            // Hash object
            let hash = hash_object(&args[2..]);
            println!("{}", hash);
        }
        "ls-tree" => list_tree_contents(&args[2..]), // List tree contents
        "write-tree" => {
            // Write tree
            let hash = write_tree(&args[2..]);
            println!("{}", hash);
        } // Write tree
        "commit-tree" => {
            // Commit tree
            let hash = commit_tree(&args[2..]);
            println!("{}", hash);
        }
        "clone" => clone_repository(&args[2..]), // Clone repository
        _ => println!("unknown command: {}", args[1]),
    }
}
