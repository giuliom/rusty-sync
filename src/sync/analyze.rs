use walkdir::WalkDir;

pub fn read_folder_content(path: &String){
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        println!(" {}", entry.path().display());
    }
}