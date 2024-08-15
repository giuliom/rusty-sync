use walkdir::WalkDir;
use std::fs::File;

use crate::rsync::crypto;

pub fn read_folder_content(path: &String){
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        println!(" {}", entry.path().display());

        let file = File::open( entry.path().display().to_string());
        if file.is_ok() {
            let hash = crypto::generate_hash(&file.unwrap());
            if hash.is_ok() {
                println!("Hash: {}", &hash.unwrap());
            }
        }
    }
}