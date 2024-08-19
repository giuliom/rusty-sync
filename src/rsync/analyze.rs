use chrono::{DateTime, Utc};
use std::fs::File;
use std::io::{Error, ErrorKind};
use std::path::Path;
use super::filesystem::{ContentTree, FolderData, FileData};
use super::crypto;

pub fn read_folder_content(path: &String) -> Result<ContentTree, std::io::Error>{
    let dir = Path::new(path);
 
    // TODO make it work for single files as well
    if dir.is_dir() == false {
        return Err(std::io::Error::new(
            ErrorKind::InvalidInput,
            format!("The path '{}' is not a directory", path),
        ));
    }   

    let metadata = std::fs::metadata(dir)?;
    let folder_name = dir.file_name().expect("Invalid path").to_str().expect("Invalid folder name").to_string();
    let last_modified: DateTime<Utc> = metadata.modified()?.into();
    
    let mut root = FolderData::new(folder_name, path.clone(), last_modified);
    let mut content = ContentTree::new(root, chrono::Utc::now());

    // TODO reorganise and properly implement algorithm
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
           //TODO
        } else if path.is_file() {
            let file = File::open( entry.path().display().to_string())?;
            let hash = crypto::generate_hash(&file)?;
            println!("Hash: {}", hash);
            content.root.file_hashes.push(hash.clone());
            let f = FileData::new(entry.file_name().to_str().unwrap().to_string(), entry.path().display().to_string(), "todo".to_string(), hash.clone(), Utc::now());
            content.files.insert(hash, f);
        }
    }
    
    Ok(content)
}

