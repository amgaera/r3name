extern crate regex;

use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;
use regex::Regex;

fn rename_path(path: &str, pattern: &Regex, replacement: &str) {
    if !pattern.is_match(path) {
        println!("Skipping `{}`", path);
        return;
    }

    if !Path::new(path).exists() {
        writeln!(io::stderr(), "Source path `{}` doesn't exist", path).unwrap();
        return;
    }

    let new_path = pattern.replace(path, replacement);

    if Path::new(&new_path).exists() {
        writeln!(io::stderr(), "Failed to rename `{}`: destination path `{}` already exists", path, new_path).unwrap();
        return;
    }

    match fs::rename(path, &new_path) {
        Ok(_) => println!("Renamed `{}` -> `{}`", path, new_path),
        Err(error) => {
            writeln!(io::stderr(), "Failed to rename `{}` -> `{}`: {}", path, new_path, error).unwrap();
            return;
        }
    };
}

fn main() {
    let pattern = env::args().nth(1).unwrap();

    let pattern = match Regex::new(&pattern) {
        Ok(regex) => regex,
        Err(error) => {
            writeln!(io::stderr(), "Invalid pattern: {}", error).unwrap();
            return;
        }
    };

    let replacement = env::args().nth(2).unwrap();

    for path in env::args().skip(3) {
        rename_path(&path, &pattern, &replacement);
    }
}
