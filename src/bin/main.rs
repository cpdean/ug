#![cfg_attr(all(test, feature = "nightly"), feature(test))] // we only need test feature when testing
extern crate regex;
extern crate getopts;
extern crate glob;

extern crate ug;

#[cfg(not(test))]
use ug::core;

#[cfg(not(test))]
use std::fs;
#[cfg(not(test))]
use std::io::prelude::*;
#[cfg(not(test))]
use std::process;

#[cfg(not(test))]
use std::collections::LinkedList;

#[cfg(not(test))]
use glob::glob;

#[cfg(not(test))]
use std::fs::File;
#[cfg(not(test))]
use std::path::{Path, PathBuf};

#[cfg(not(test))]
use regex::Regex;

#[cfg(not(test))]
use getopts::{Options, Matches};
#[cfg(not(test))]
use std::env;

/// walk downwards from the current path and return
/// a list of paths to files
#[cfg(not(test))]
fn get_files(this_path: &Path, ignores: &Vec<PathBuf>) -> Vec<PathBuf>{
    let contents = fs::read_dir(this_path).unwrap();
    let mut output: Vec<PathBuf> = Vec::new();
    //let ignores = vec![Path::new("./.git")];

    for path in contents {
        let p = path.unwrap().path();
        if ignores.contains(&p){
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

    return output;
}

#[cfg(not(test))]
fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} PATTERN [options]", program);
    print!("{}", opts.usage(&brief));
}

#[cfg(not(test))]
fn lines_of(file: &str) -> Vec<String> {
    let mut buffer = String::new();
    // TODO: maybe move this side effect out, hand it a
    //       stream of lines or otherwise opened file
    let mut f = File::open(file).unwrap();
    let _ = f.read_to_string(&mut buffer).unwrap();
    buffer.lines().map(ToOwned::to_owned).collect()
}

#[cfg(not(test))]
fn get_ignored_files_from_config() -> LinkedList<PathBuf> {
    let mut o = LinkedList::new();
    for line in lines_of(".gitignore") {
        o.push_back(Path::new(&line).to_path_buf())
    }
    o
}

#[cfg(not(test))]
fn get_things_you_should_ignore() -> Vec<PathBuf> { 

    let mut heynow = get_ignored_files_from_config();

    let known_files_to_ignore = glob(".git/*")
        .unwrap()
        .map(|x| x.unwrap());

    heynow.extend(known_files_to_ignore);
    let mut fixed: Vec<PathBuf> = Vec::new();

    let jerk: Vec<_> = heynow.into_iter()
        .map(|x| {
            let mut guy = PathBuf::from(".");
            guy.push(x.as_path());
            guy
        }).collect();

    fixed.extend(jerk);
    fixed
}

#[cfg(not(test))]
fn get_opts() -> Result<(String, Matches), String> {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("l", "list-files", "list only files that contain the pattern");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    let pattern = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return Err("not enough args".to_string())
    };
    Ok((pattern, matches))

}

#[cfg(not(test))]
type FileResult = (PathBuf, Vec<(usize, String)>);

/// given the matches, generate output as a
/// stream of lines that will then be printed later
#[cfg(not(test))]
fn display_output(results: Vec<FileResult>, opts: &Matches) -> Vec<String> {
    let mut o: Vec<String> = Vec::new();
    for (pat, linz) in results {
        if !linz.is_empty() {
            o.push(format!("{}", pat.display()));
            if !opts.opt_present("l") {
                for (line_num, lin) in linz{
                    o.push(format!("{}:{}", line_num, lin));
                }
                o.push(format!(""));
            }
        }
    }
    o
}

#[cfg(not(test))]
fn main() {

    let (re, opts) = match get_opts() {
        Ok((p, o)) => { (Regex::new(&p).unwrap(), o) },
        Err(_) => { process::exit(1) },
    };

    let fixed = get_things_you_should_ignore();

    let results: Vec<FileResult> = get_files(Path::new("."), &fixed).into_iter()
        .map(|p| {
            let such_lines = core::matching_lines(&p, &re);
            (p, such_lines)
        }).collect();

    for l in display_output(results, &opts) {
        println!("{}", l);
    }

}
