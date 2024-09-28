use std::{env, fs};
use std::path::PathBuf;

mod cli;

fn main() {
    let path = env::args()
        .nth(1)
        .expect("A file path arg was never provided");
    
    let mut sigma_path = PathBuf::from(&path);

    if sigma_path.set_extension("cpp") {
        fs::rename(path, sigma_path).unwrap()
    }
}
