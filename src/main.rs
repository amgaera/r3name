extern crate docopt;
extern crate regex;

use std::fmt;
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

#[derive(Debug)]
enum RenameError<'a> {
    RegexDoesNotMatch(&'a Regex, &'a str),
    SourceDoesNotExist(&'a str),
    DestinationExists(String),
    FsError(io::Error)
}

impl<'a> fmt::Display for RenameError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RenameError::RegexDoesNotMatch(pattern, path) =>
                write!(f, "path `{}` doesn't match regex `{}`)", path, pattern),
            RenameError::SourceDoesNotExist(path) =>
                write!(f, "source path `{}` doesn't exist", path),
            RenameError::DestinationExists(ref path) =>
                write!(f, "destination path `{}` already exists", path),
            RenameError::FsError(ref error) => write!(f, "{}", error)
        }
    }
}

fn rename_path<'a>(path: &'a str, pattern: &'a Regex, replacement: &str, dry_run: bool)
                   -> Result<String, RenameError<'a>> {
    if !pattern.is_match(path) {
        return Err(RenameError::RegexDoesNotMatch(pattern, path));
    }

    if !Path::new(path).exists() {
        return Err(RenameError::SourceDoesNotExist(path));
    }

    let new_path = pattern.replace(path, replacement);

    if Path::new(&new_path).exists() {
        return Err(RenameError::DestinationExists(new_path));
    }

    if dry_run {
        return Ok(new_path);
    }

    return fs::rename(path, &new_path).map(|()| new_path).map_err(RenameError::FsError);
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
        match rename_path(path, &pattern, replacement, dry_run) {
            Ok(new_path) => {
                let prefix = if dry_run { "Would rename" } else { "Renamed" };
                println!("{} `{}` -> `{}`", prefix, path, new_path);
            },
            Err(RenameError::RegexDoesNotMatch(_, _)) =>
                println!("Skipping `{}` (doesn't match regex `{}`)", path, pattern),
            Err(error) => writeln!(io::stderr(), "Failed to rename `{}`: {}", path, error).unwrap()
        }
    }
}
