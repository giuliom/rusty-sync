use std::fs::{self, read};
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
    let source = &args[2];
    let dest = &args[4];

   sync(source, dest);
}

fn sync(source: &String, dest: &String) {

    let current_dir = std::env::current_dir().unwrap();
    println!("Current directory: {}", current_dir.display());

    println!("Source directory: {source}");
    read_folder_content(source);
    println!("Destination directory: {dest}");
    read_folder_content(dest);
}

fn read_folder_content(path: &String){
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        println!(" {}", entry.path().display());
    }
}


