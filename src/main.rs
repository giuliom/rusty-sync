use std::fs;
use std::path::{Path, PathBuf};
use std::io::{self, ErrorKind};
use walkdir::WalkDir;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!("Usage: {} -s <source_directory> -d <destination_directory>", args[0]);
        std::process::exit(1);
    }

    // TODO
    let source = &args[1];
    let dest = &args[2];

   sync(source, dest);
}

fn sync(source: &String, dest: &String) {

    println!("Source directory: {}", source);
    println!("Destination directory: {}", dest);
}



