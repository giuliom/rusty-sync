use super::analyze;

pub fn sync(source: &String, dest: &String) {

    let current_dir = std::env::current_dir().unwrap();
    println!("Current directory: {}", current_dir.display());

    println!("Source directory: {source}");
    let source = analyze::read_folder_content(source);
    println!("Source: \n{:#?}", source);
    println!("Destination directory: {dest}");
    let destination = analyze::read_folder_content(dest);
    println!("Destination: \n{:#?}", destination);
}