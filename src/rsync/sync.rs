use super::analyze::{self, Action};


pub fn sync(source: &String, dest: &String) {

    let current_dir = std::env::current_dir().unwrap();
    println!("Current directory: {}", current_dir.display());

    println!("Source directory: {source}");
    println!("Destination directory: {dest}");

    let result = analyze::compare(source, dest);
    let mut actions: Vec<Action> = Vec::new();
    
    match result {
        Ok(acts) => {
            actions = acts;
        }
        Err(e) => {
            eprintln!("Sync failed with error: {}", e);
            return;
        }
    }

    // TODO implement actions
}