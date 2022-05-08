use tpng::Image;
use std::env;
use std::path::{Path, PathBuf};
use std::fs;
use std::error::Error;

fn run() -> Result<(), Box<dyn Error>> {
    let path_string = env::args().nth(1).ok_or("usage: tpng <img_path>")?;
    let path = fs::canonicalize(PathBuf::from(&path_string))?;
    println!("{}", Image::new(&path)?);
    Ok(())
}

fn main() { if let Err(e) = run() { eprintln!("Error: {e}"); } }

