use std::io::prelude::*;

#[cfg(not(test))]
use std::fs::File;

#[cfg(not(test))]
use std::path::PathBuf;

use regex::Regex;

/// get matching lines from a path
#[cfg(not(test))]
pub fn matching_lines(p: &PathBuf, pattern: &Regex) ->  Vec<(usize, String)> {
    let mut buffer = String::new();
    let mut f = File::open(&p).unwrap();
    match f.read_to_string(&mut buffer) {
        Ok(yay_read) => yay_read,
        Err(_) => 0,
    };
    return _matching_lines(buffer, pattern);
}

pub fn _matching_lines(contents: String, pattern: &Regex) -> Vec<(usize, String)> {
    contents.lines()
        .enumerate()
        .filter(|&(_, x)| pattern.is_match(&x))
        .map(|(i, x)| (i + 1, x.to_owned()))
        .collect()
}

#[test]
fn the_matchline_finds_something_and_gives_line_number() {
    let file_to_search: String =
        "first line
        second line
        something
        nothing
        also nothing great".to_string();
    let to_find = Regex::new("something").unwrap();
    let results: Vec<(usize, String)> = _matching_lines(file_to_search, &to_find);
    assert_eq!(results.len(), 1);

    assert_eq!(results[0], (3, "        something".to_string()));
}

#[test]
fn matching_lines_two_things() {
    let file_to_search: String =
        "first line
        second line
        thing one
        thing two
        junk line".to_string();
    let to_find = Regex::new("thing").unwrap();
    let results: Vec<(usize, String)> = _matching_lines(file_to_search, &to_find);
    assert_eq!(results.len(), 2);

    assert_eq!(results[0], (3, "        thing one".to_string()));
    assert_eq!(results[1], (4, "        thing two".to_string()));
}
