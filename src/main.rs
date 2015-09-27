extern crate regex;
extern crate getopts;

use std::fs;
use std::io::prelude::*;
use std::io::BufReader;

use std::fs::File;
use std::path::{Path,PathBuf};

use regex::Regex;

use getopts::Options;
use std::env;

/// buffered_reader_search tries to use a BufReader for looking
/// at the contents of files instead of using the file's
/// trait method read_to_string.  The reasoning was that
/// reading an entire file to a string might be slow as
/// it would need to get to the end of the file before it can
/// split it by lines and conduct the search.
///
/// turns out that this is about 10x slower with debug compilation
/// and 2x slower on a release build
#[allow(dead_code)]
fn buffered_reader_search(this_path: &Path, for_this: &Regex) {
    let contents = fs::read_dir(this_path).unwrap();
    for path in contents {
        let p = path.unwrap().path();
        if fs::metadata(&p).unwrap().is_dir() {
            buffered_reader_search(&p, for_this);
        } else if fs::metadata(&p).unwrap().is_file() {
            //println!("looking at {}", &p.display());
            let f = File::open(&p).unwrap();
            let reader = BufReader::new(f);
            let matching_lines = reader.lines().filter(
                    |x| x.is_ok()
                ).map(|x| x.unwrap())
                .filter(|x| for_this.is_match(x));
            for l in matching_lines {
                println!("{}", l)
            }
        }
    }
}

#[allow(dead_code)]
fn print_files_matching(this_path: &Path, for_this: &Regex) {
    let contents = fs::read_dir(this_path).unwrap();
    for path in contents {
        let p = path.unwrap().path();
        if fs::metadata(&p).unwrap().is_dir() {
            print_files_matching(&p, for_this);
        } else if fs::metadata(&p).unwrap().is_file() {
            //println!("looking at {}", &p.display());
            let mut f = File::open(&p).unwrap();
            let mut buffer = String::new();
            match f.read_to_string(&mut buffer) {
                Ok(yay_read) => yay_read,
                Err(_) => 0,
            };
            let matching_lines = buffer.lines().filter(|&x| for_this.is_match(x));
            for l in matching_lines {
                println!("{}", l)
            }
        }
    }
}

/// walk downwards from the current path and return
/// a list of paths to files
fn get_files(this_path: &Path) -> Vec<PathBuf>{
    let contents = fs::read_dir(this_path).unwrap();
    let mut output: Vec<PathBuf> = Vec::new();

    for path in contents {
        let p = path.unwrap().path();
        if fs::metadata(&p).unwrap().is_dir() {
            for child_path in get_files(&p) {
                output.push(child_path)
            }
        } else if fs::metadata(&p).unwrap().is_file() {
            output.push(p)
        }
    }

    return output;
}

/// print the lines that match
fn matching_lines(p: PathBuf, pattern: &Regex) {
    let mut buffer = String::new();
    // TODO: maybe move this side effect out, hand it a
    //       stream of lines or otherwise opened file
    let mut f = File::open(&p).unwrap();
    match f.read_to_string(&mut buffer) {
        Ok(yay_read) => yay_read,
        Err(_) => 0,
    };
    let m_lines = buffer.lines().filter(|&x| pattern.is_match(x));
    for l in m_lines {
        println!("{}", l)
    }
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} PATTERN [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let opts = Options::new();
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    let pattern = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };

    let ref re = Regex::new(&pattern).unwrap();
    for p in get_files(Path::new(".")) {
        matching_lines(p, re);
    }
    //print_files_matching(Path::new("."), re);
    //buffered_reader_search(Path::new("."), re);
}
