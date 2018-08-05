extern crate getopts;
extern crate glob;
extern crate regex;

extern crate ug;

use ug::core;
use ug::io;

use std::fs;
use std::io::prelude::*;
use std::process;

use std::collections::LinkedList;

use glob::glob;

use std::fs::File;
use std::path::{Path, PathBuf};

use regex::Regex;

use std::env;

/// walk downwards from the current path and return
/// a list of paths to files
fn get_files(this_path: &Path, ignores: &[PathBuf]) -> Vec<PathBuf> {
    let contents = fs::read_dir(this_path).unwrap();
    let mut output: Vec<PathBuf> = Vec::new();

    for path in contents {
        let p = path.unwrap().path();
        if ignores.contains(&p) {
            continue;
        }
        if fs::metadata(&p).unwrap().is_dir() {
            for child_path in get_files(&p, ignores) {
                output.push(child_path)
            }
        } else if fs::metadata(&p).unwrap().is_file() {
            output.push(p)
        }
    }

    output
}

fn lines_of(file: &str) -> Vec<String> {
    let mut buffer = String::new();
    // TODO: maybe move this side effect out, hand it a
    //       stream of lines or otherwise opened file
    let mut f = File::open(file).unwrap();
    let _ = f.read_to_string(&mut buffer).unwrap();
    buffer.lines().map(ToOwned::to_owned).collect()
}

fn get_ignored_files_from_config() -> LinkedList<PathBuf> {
    let mut o = LinkedList::new();
    for line in lines_of(".gitignore") {
        o.push_back(Path::new(&line).to_path_buf())
    }
    o
}

fn get_things_you_should_ignore() -> Vec<PathBuf> {
    let mut gitignored = get_ignored_files_from_config();

    let known_files_to_ignore = glob(".git/*").unwrap().map(|x| x.unwrap());

    gitignored.extend(known_files_to_ignore);

    // prefix files gathered with the search root
    gitignored
        .into_iter()
        .map(|x| {
            let mut new_path = PathBuf::from(".");
            new_path.push(x.as_path());
            new_path
        }).collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let (re, path, opts) = match io::get_opts(&args) {
        Ok((pattern, path, o)) => (Regex::new(&pattern).unwrap(), path, o),
        Err(_) => process::exit(1),
    };

    let files_to_ignore = get_things_you_should_ignore();

    let results: Vec<core::FileResult> = get_files(Path::new(&path), &files_to_ignore)
        .into_iter()
        .map(|p| {
            let such_lines = core::matching_lines(&p, &re);
            (p, such_lines)
        }).collect();

    for l in io::display_output(results, &opts) {
        println!("{}", l);
    }
}
