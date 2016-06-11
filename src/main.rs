extern crate docopt;
extern crate regex;

use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;
use docopt::Docopt;
use regex::Regex;

const USAGE: &'static str = "
Usage: r3name --pattern <pattern> --replacement <replacement> [--dry-run] <path>...
       r3name -h | --help
       r3name --version

Options:
    -h --help                    Show this screen.
    --version                    Show version.
    --pattern <pattern>          Pattern to match in the provided paths
    --replacement <replacement>  String used to replace pattern matches
    --dry-run                    Show what would be renamed, but don't rename anything.
";

const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

fn rename_path(path: &str, pattern: &Regex, replacement: &str, dry_run: bool) {
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

    if dry_run {
        println!("Would rename `{}` -> `{}`", path, new_path);
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
    let args = Docopt::new(USAGE).and_then(|d| d.parse()).unwrap_or_else(|e| e.exit());

    if args.get_bool("--version") {
        println!("r3name {}", VERSION.unwrap_or("unknown"));
        return;
    }

    let pattern = args.get_str("--pattern");
    let pattern = match Regex::new(pattern) {
        Ok(regex) => regex,
        Err(error) => {
            writeln!(io::stderr(), "Invalid pattern: {}", error).unwrap();
            return;
        }
    };

    let replacement = args.get_str("--replacement");
    let dry_run = args.get_bool("--dry-run");

    for path in args.get_vec("<path>") {
        rename_path(path, &pattern, replacement, dry_run);
    }
}
