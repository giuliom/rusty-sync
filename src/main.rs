use std::fs::{self, read};
use std::path::{Path, PathBuf};
use std::io::{self, ErrorKind};
use std::env;

mod sync;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!("Usage: {} -s <source_directory> -d <destination_directory>", args[0]);
        std::process::exit(0);
    }

    // TODO
    let source = &args[2];
    let dest = &args[4];

    println!("Sync Started");
    sync::sync(source, dest);
    println!("Sync Completed");
}


