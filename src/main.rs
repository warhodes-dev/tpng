use tpng::run;
use std::env;
use std::path::{Path, PathBuf};
use std::fs;

fn main() {

    if let Some(path_string) = env::args().nth(1) {

        match fs::canonicalize(PathBuf::from(&path_string)) {
            Ok(full_path) => {
                let path = Path::new(&full_path);
                if let Err(e) = run(path) {
                    println!("Error: {}", e);
                }
            }
            Err(e) => {
                println!("Error: {}", e)
            }
        };
    } else {
        println!("usage: trender <imgpath>")
    }
}
