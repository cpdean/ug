extern crate regex;

use std::fs;
use std::io::prelude::*;
use std::io::BufReader;

use std::fs::File;
use std::path::Path;

use regex::Regex;

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

fn main() {
    let ref re = Regex::new(r"f.lter").unwrap();
    //print_files_matching(Path::new("."), re);
    buffered_reader_search(Path::new("."), re);
}
