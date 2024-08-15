use std::env;

mod analyze;
mod crypto;
mod filesystem;

pub fn sync(source: &String, dest: &String) {

    let current_dir = std::env::current_dir().unwrap();
    println!("Current directory: {}", current_dir.display());

    println!("Source directory: {source}");
    analyze::read_folder_content(source);
    println!("Destination directory: {dest}");
    analyze::read_folder_content(dest);
}