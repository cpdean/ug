extern crate ug;

use std::path::{Path};
use ug::io;

#[test]
fn test_file_only_printer() {
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
    assert_eq!(vec![format!("test_file.txt")], output);
}

#[test]
fn test_regular_search_display() {
    let p = Path::new("test_file.txt").to_path_buf();
    let m = vec![(1, format!("a match"))];
    let args = vec![format!("self"),
                    format!("beh")];
    let (_, opts) = match io::get_opts(args) {
        Ok((_, o)) => { (1, o) },
        Err(_) => { panic!("sure hope not") },
    };

    let file_result = vec![(p, m)];
    let output = io::display_output(file_result, &opts);
    assert_eq!(
        vec![
            format!("test_file.txt"),
            format!("1:a match"),
            format!(""),
        ],
        output
    );
}
