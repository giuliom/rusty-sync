use chrono;
use std::io::ErrorKind;
use std::path::Path;
use super::filesystem::{ContentTree, FileData, FolderData};

pub enum ActionType {
    ADD,
    MODIFY,
    DELETE,
}

pub struct Action {
    pub category: ActionType,
}

pub fn read_folder_content(path: &String) -> Result<ContentTree, std::io::Error>{
    let dir = Path::new(path);
 
    // TODO make it work for single files as well
    if dir.is_dir() == false {
        return Err(std::io::Error::new(
            ErrorKind::InvalidInput,
            format!("The path '{}' is not a directory", path),
        ));
    }
  
    let root = FolderData::from_path(dir)?;
    let mut content = ContentTree::new(root, chrono::Utc::now());

    let mut folders: Vec<&mut FolderData> = Vec::new();
    folders.push(&mut content.root);

    while folders.len() > 0 {
        let folder = folders.pop().unwrap();
        let mut subfolders: Vec<FolderData> = Vec::new();

        // TODO reorganise and properly implement algorithm
        for entry in std::fs::read_dir(&folder.path)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                let sub_folder = FolderData::from_dir(&entry)?;
                subfolders.push(sub_folder);
            } else if path.is_file() {
                let file_data = FileData::from_dir(&entry)?;
                folder.file_hashes.push(file_data.hash.clone());
                content.files.insert(file_data.hash.clone(), file_data);
            }
        }

        folder.subfolders = subfolders;
        
        for fd in &mut folder.subfolders {
            folders.push(fd);
        }
    }

    Ok(content)
}


pub fn compare(source: &String, dest: &String) -> Result<Vec<Action>, std::io::Error>{
    println!("Analaysis of source and destination folders");

    let source = read_folder_content(source)?;
    println!("Source: \n{:#?}", source);

    let destination = read_folder_content(dest)?;
    println!("Destination: \n{:#?}", destination);

    let actions: Vec<Action> = Vec::new();

    // TODO

    Ok(actions)
}
