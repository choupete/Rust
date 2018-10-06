use std::fs::ReadDir;
use regex::Regex;
use std::path::PathBuf;
extern crate regex;
use std::env;
use std::io;
use std::io::{Error, ErrorKind};

// Executable
fn main() -> Result<(),io::Error> {
	match parse_args(env::args().collect()) {
		Some(x) => {
			let (pattern, p) = x;
			let all_directories = get_directories(p.read_dir()?)?;
			let matched_vec = get_matches(all_directories, pattern);
			for entry in matched_vec {
			println!("Match Found: {}", &entry.to_string_lossy());
			}
			return Ok(())
		},
		None => {
			return Err(Error::new(ErrorKind::InvalidInput, "Invalid Arguments!"));
		},
	}
}



fn get_matches(paths: Vec<PathBuf>, pattern: Regex) -> Vec<PathBuf> {
	let mut path_vec: Vec<PathBuf> = Vec::<PathBuf>::new();
	for p in paths {
		if pattern.is_match(&p.to_string_lossy()) {
			path_vec.push(p);
		}
	};
	path_vec
}

fn get_directories(dir: ReadDir) -> Result<Vec<PathBuf>, io::Error> {
	let mut path_vec: Vec<PathBuf> = Vec::<PathBuf>::new();
	for entry in dir {
		match entry {
			Ok(x) => {
				if x.path().is_dir() {
					let dir_arg = x.path().read_dir()?;
					path_vec.extend(get_directories(dir_arg)?);
				} else {
					path_vec.push(x.path());
				}
			},
			Err(e) => return Err(e),
		}
	}
	Ok(path_vec)
}

fn parse_args(args: Vec<String>) -> Option<(Regex, PathBuf)> {
	match args.as_slice() {
		[_, args2, args3] => {
			let expression = Regex::new(args2);
			match expression {
				Ok(x) => Some((x, PathBuf::from(args3))),
				Err(_) => {
					eprintln!("Invalid Regex");
					None
				}
			}
		},
		_ => {
			None
		}
	}
}