use std::{env, fs};

fn main() {

    let home = env::var("HOME").unwrap_or_else(|_| env::var("USERPROFILE").unwrap());

    let paths = fs::read_dir(home).unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
}