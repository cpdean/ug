extern crate ug;

use std::path::{Path};
use ug::io;

#[test]
fn test_only_file_printer() {
    let p = Path::new("test_file.txt").to_path_buf();
    let m = vec![(1, "a match".to_string())];
    let args = vec!["self".to_string(),
                    "beh".to_string(),
                    "-l".to_string()];
    let (_, opts) = match io::get_opts(args) {
        Ok((_, o)) => { (1, o) },
        Err(_) => { panic!("at the disco") },
    };

    let file_result = vec![(p, m)];
    let output = io::display_output(file_result, &opts);
    assert_eq!(1, output.len());
    assert_eq!("test_file.txt".to_string(), output[0]);
}

#[test]
fn test_regular_search_display() {
    let p = Path::new("test_file.txt").to_path_buf();
    let m = vec![(1, "a match".to_string())];
    let args = vec!["self".to_string(),
                    "beh".to_string()];
    let (_, opts) = match io::get_opts(args) {
        Ok((_, o)) => { (1, o) },
        Err(_) => { panic!("at the disco") },
    };

    let file_result = vec![(p, m)];
    let output = io::display_output(file_result, &opts);
    assert_eq!(3, output.len());
    assert_eq!("test_file.txt".to_string(), output[0]);
    assert_eq!("1:a match".to_string(), output[1]);
}
