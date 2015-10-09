extern crate regex;
extern crate getopts;
extern crate glob;

use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
use std::process;

use std::collections::LinkedList;

use glob::glob;

use std::fs::File;
use std::path::{Path, PathBuf};

use regex::Regex;

use getopts::{Options, Matches};
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

/// get matching lines from a path
fn matching_lines(p: &PathBuf, pattern: &Regex) ->  Vec<(usize, String)> {
    let mut buffer = String::new();
    // TODO: maybe move this side effect out, hand it a
    //       stream of lines or otherwise opened file
    let mut f = File::open(&p).unwrap();
    match f.read_to_string(&mut buffer) {
        Ok(yay_read) => yay_read,
        Err(_) => 0,
    };
    let m_lines: Vec<(usize, String)> = buffer.lines()
        .enumerate()
        .filter(|&(i, x)| pattern.is_match(&x))
        .map(|(i, x)| (i + 1, x.to_owned()))
        .collect();
    return m_lines;
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} PATTERN [options]", program);
    print!("{}", opts.usage(&brief));
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

type FileResult = (PathBuf, Vec<(usize, String)>);

fn display_them(results: Vec<FileResult>, opts: &Matches) {
    for (pat, linz) in results {
        if !linz.is_empty() {
            println!("{}", pat.display());
            for (line_num, lin) in linz{
                println!("{}:{}", line_num, lin)
            }
            println!("")
        }
    }
}

fn main() {

    let (re, opts) = match get_opts() {
        Ok((p, o)) => { (Regex::new(&p).unwrap(), o) },
        Err(_) => { process::exit(1) },
    };

    let fixed = get_things_you_should_ignore();

    if opts.opt_present("l") {
        println!("wot m8???");
        return;
    }
    else {
        let results: Vec<FileResult> = get_files(Path::new("."), &fixed).into_iter()
            .map(|p| {
                let such_lines = matching_lines(&p, &re);
                (p, such_lines)
            }).collect();

        display_them(results, &opts);

    }


}
