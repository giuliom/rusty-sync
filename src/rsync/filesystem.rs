use chrono::{DateTime, Utc};
use std::{collections::HashMap, fs::{DirEntry, File}, path::Path, time::SystemTime};
use super::crypto;

#[derive(Clone, Debug)]
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

    pub fn from_dir(dir: &DirEntry) -> std::io::Result<FolderData> {
        assert!(dir.path().is_dir());

        let fd = FolderData::from_path(dir.path().as_path())?;
        Ok(fd)
    }

    pub fn from_path(path: &Path) -> std::io::Result<FolderData> {
        assert!(path.is_dir());

        let metadata = std::fs::metadata(path)?;
        let name = path.get_name();
        let path_string = path.to_str().unwrap().to_string();
        let last_modified: DateTime<Utc> = metadata.modified().unwrap_or(SystemTime::now()).into();

        let fd = FolderData::new(name, path_string, last_modified);
        Ok(fd)
    }
}

#[derive(Clone, Debug)]
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

    pub fn from_dir(dir: &DirEntry) -> std::io::Result<FileData> {
        assert!(dir.path().is_file());
        let file = File::open(dir.path().display().to_string())?;
        let metadata = file.metadata()?;

        let name = dir.get_name();
        let path = dir.path().to_str().unwrap().to_string();
        let extension = dir.path().extension().unwrap().to_str().unwrap().to_string();
        let hash = crypto::generate_hash(&file)?;
        let last_modified =  metadata.modified().unwrap_or(SystemTime::now()).into();

        let fd = FileData::new(name, path, extension, hash, last_modified);
        Ok(fd)
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

pub trait GetName {
    fn get_name(&self) -> String;
}

impl GetName for Path {
    fn get_name(&self) -> String {
        self.file_name().unwrap().to_str().unwrap().to_string()
    }
}

impl GetName for DirEntry {
    fn get_name(&self) -> String {
        self.file_name().to_str().unwrap().to_string()
    }
}