use std::fs;
use std::io::prelude::*;
use std::io::BufReader;

use std::fs::File;
use std::path::Path;

fn buffered_reader_search(this_path: &Path, for_this: &str) {
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
                .filter(|x| x.contains(for_this));
            for l in matching_lines {
                println!("{}", l)
            }
        }
    }
}

fn print_files_matching(this_path: &Path, for_this: &str) {
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
            let matching_lines = buffer.lines().filter(|&x| x.contains(for_this));
            for l in matching_lines {
                println!("{}", l)
            }
        }
    }
}

fn main() {
    print_files_matching(Path::new("."), "metadata");
    //buffered_reader_search(Path::new("."), "metadata");
}
