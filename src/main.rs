use std::{env::args_os, path::PathBuf};

fn main() {
    // The path to the source file.
    let path: PathBuf = args_os()
        .nth(1)
        .unwrap_or_else(|| {
            eprintln!("Usage: sea <path>");
            std::process::exit(1)
        })
        .into();

    println!("Reading from {}", path.display());
}
