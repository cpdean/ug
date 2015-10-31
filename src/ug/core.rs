

use std::io::prelude::*;
use std::fs::File;
use std::path::{Path, PathBuf};

use regex::Regex;

/// get matching lines from a path
#[cfg(not(test))]
pub fn matching_lines(p: &PathBuf, pattern: &Regex) ->  Vec<(usize, String)> {
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

#[test]
fn it_works() {
    assert!(1 == 1);
}
