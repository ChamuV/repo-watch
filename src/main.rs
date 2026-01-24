use std::{env, fs};
use walkdir::WalkDir;

fn main() {
    let home = env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .expect("Could not determine home directory");

    let paths = fs::read_dir(&home).expect("Failed to read home directory");
 
    for path in paths {
        let path = path.expect("Invalid directory entry").path();

        // println!("Name: {}", path.unwrap().path().display())
        for entry in WalkDir::new(&path) 
            .max_depth(2)
            .into_iter()
            .filter_map(|e| e.ok()) {
                if entry.file_type().is_dir() {
                    let folder = entry.path();
                    let git_folder_path = folder.join(".git"); 
                    if fs::metadata(&git_folder_path).is_ok() {
                        println!("Found Git repository: {}", folder.display());
                    }
                }
            }
        
    }

}