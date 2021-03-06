use tpng::Image;
use std::env;
use std::path::PathBuf;
use std::fs;
use std::error::Error;

fn run() -> Result<(), Box<dyn Error>> {
    let path_arg = env::args().nth(1).ok_or("invalid argument")?;
    let path = fs::canonicalize(PathBuf::from(&path_arg))?;
    let img = Image::new(&path)?;
    println!("{}", img);
    Ok(())
}

fn main() { if let Err(e) = run() { eprintln!("Error: {}", e); } }

