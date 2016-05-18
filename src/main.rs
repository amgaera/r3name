extern crate regex;

use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;
use regex::Regex;

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

    for arg in env::args().skip(3) {
        if !pattern.is_match(&arg) {
            println!("Skipping `{}`", arg);
            continue;
        }

        if !Path::new(&arg).exists() {
            writeln!(io::stderr(), "Source path `{}` doesn't exist", arg).unwrap();
            return;
        }

        let new_name = pattern.replace(&arg, &*replacement);

        if Path::new(&new_name).exists() {
            writeln!(io::stderr(), "Failed to rename `{}`: destination path `{}` already exists", arg, new_name).unwrap();
            return;
        }

        match fs::rename(&arg, &new_name) {
            Ok(_) => println!("Renamed `{}` -> `{}`", arg, new_name),
            Err(error) => {
                writeln!(io::stderr(), "Failed to rename `{}` -> `{}`: {}", arg, new_name, error).unwrap();
                return;
            }
        };
    }
}
