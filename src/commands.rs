use std::fs;
use std::io::prelude::*;

use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use sha1::{Digest, Sha1};

pub fn initialize_git_repository() {
    fs::create_dir(".git").expect("Failed to create .git directory");
    fs::create_dir(".git/objects").expect("Failed to create .git/objects directory");
    fs::create_dir(".git/refs").expect("Failed to create .git/refs directory");
    fs::write(".git/HEAD", "ref: refs/heads/main\n").expect("Failed to create .git/HEAD file");
}

pub fn print_git_object_contents(args: &[String]) {
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

pub fn hash_object(args: &[String]) {
    if args.len() != 2 {
        println!("usage: my-git hash-object -w <file>");
        std::process::exit(1);
    }

    // Parsing arguments
    let write_flag = &args[0];
    let path = &args[1];

    // Reading object
    let content: Vec<u8> = fs::read(format!("./{}", path)).expect("Failed to read object");

    // Creating object header
    let mut content_with_header = format!("blob {}\0", content.len()).as_bytes().to_vec();
    content_with_header.extend(&content);

    // Hashing object
    let mut hasher = Sha1::new();
    hasher.update(&content_with_header);
    let hash = hasher.finalize();
    let hash_string = format!("{:x}", hash);

    // Creating object directory
    fs::create_dir(format!(".git/objects/{}", &hash_string[..2]))
        .expect("Failed to create object directory");

    // Compressing object
    let mut compressed_data = Vec::new();
    let mut encoder = ZlibEncoder::new(&mut compressed_data, flate2::Compression::default());
    encoder
        .write_all(&content_with_header)
        .expect("Failed to compress object");
    let compressed_content = encoder.finish().expect("Failed to finish compression");

    // Writing object
    if write_flag == "-w" {
        fs::write(
            format!(".git/objects/{}/{}", &hash_string[..2], &hash_string[2..]),
            &compressed_content,
        )
        .expect("Failed to write object");
    }

    // Printing hash
    println!("{}", hash_string);
}

pub fn list_tree_contents(args: &[String]) {
    if args.len() != 2 {
        println!("usage: my-git ls-tree --name-only <hash>");
        std::process::exit(1);
    }

    let hash = &args[1];

    let content: Vec<u8> = fs::read(format!(".git/objects/{}/{}", &hash[..2], &hash[2..]))
        .expect("Failed to read object");

    let mut z_lib_decoder = ZlibDecoder::new(&content[..]);
    let mut contents_decoded = vec![];
    z_lib_decoder.read_to_end(&mut contents_decoded).unwrap();

    let mut i = 0;
    let mut hit_first_null: bool = false;
    while i < contents_decoded.len() {
        if !hit_first_null {
            if contents_decoded[i] == b'\0' {
                hit_first_null = true;
            }
            i += 1;
            continue;
        }
        // Read the mode
        let mut mode_end = i;
        while contents_decoded[mode_end] != b' ' {
            mode_end += 1;
        }
        let mode = std::str::from_utf8(&contents_decoded[i..mode_end]).unwrap();
        i = mode_end + 1;
        // Parses the type of the mode
        let _mode_type = match mode {
            "100644" => "blob", // Regular file
            "100755" => "blob", // Executable file
            "120000" => "blob", // Symbolic link
            "40000" => "tree",  // Directory
            _ => "unknown",
        };
        // Read the filename
        let mut name_end = i;
        while contents_decoded[name_end] != b'\0' {
            name_end += 1;
        }
        let name = std::str::from_utf8(&contents_decoded[i..name_end]).unwrap();
        i = name_end + 1;
        // Read the SHA-1 hash
        let hash = &contents_decoded[i..i + 20];
        let _hash = hash
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>();
        i += 20;
        println!("{}", name);
        // if is_name_only {
        // println!("{}", name);
        // } else {
        //     let mode = format!("{:06}", mode.parse::<u32>().unwrap());
        //     println!("{} {} {}\t{}", mode, mode_type, hash, name);
        // }
    }
}

pub fn write_tree(args: &[String]) {
    if args.len() != 0 {
        println!("usage: my-git write-tree");
        std::process::exit(1);
    }

    println!("write_tree: {:?}", args);
}
