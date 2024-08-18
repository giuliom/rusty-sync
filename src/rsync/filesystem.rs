use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[derive(Debug)]
pub struct FolderData {
    pub name: String,
    pub path: String,
    pub last_modified: DateTime<Utc>,
    pub file_hashes: Vec<String>,
    pub subfolders: Vec<FolderData>,
}

impl FolderData {
    pub fn new(name: String, path: String, last_modified: DateTime<Utc>) -> FolderData {
        FolderData{name, path, last_modified, file_hashes: Vec::new(), subfolders: Vec::new()}
    }
}

#[derive(Debug)]
pub struct FileData {
    pub name: String,
    pub path: String,
    pub extension: String,
    pub hash: String,
    pub last_modified: DateTime<Utc>,
}

impl FileData {
    pub fn new(name: String, path: String, extension: String, hash: String, last_modified: DateTime<Utc>) -> FileData {
        FileData{name, path, extension, hash, last_modified}
    }
}

#[derive(Debug)]
pub struct ContentTree {
    pub root: FolderData,
    pub files: HashMap<String, FileData>,
    pub last_analysis: DateTime<Utc>,
}

impl ContentTree {
    pub fn new(root: FolderData, last_analysis: DateTime<Utc>) -> ContentTree {
        ContentTree{root, files: HashMap::new(), last_analysis}
    }
}