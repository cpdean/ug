use getopts::{Matches, Options};

use core;

enum DisplayMode {
    OnlyFiles,
    CountMatches,
    Regular,
}

fn get_display_mode(opts: &Matches) -> DisplayMode {
    if opts.opt_present("l") {
        return DisplayMode::OnlyFiles;
    }
    if opts.opt_present("c") {
        return DisplayMode::CountMatches;
    }
    DisplayMode::Regular
}

/// given the matches, generate output as a
/// stream of lines that will then be printed later
pub fn display_output(results: Vec<core::FileResult>, opts: &Matches) -> Vec<String> {
    let mut o: Vec<String> = Vec::new();
    for (pat, matching_lines) in results {
        if !matching_lines.is_empty() {
            match get_display_mode(&opts) {
                DisplayMode::OnlyFiles => {
                    o.push(format!("{}", pat.display()));
                }
                DisplayMode::CountMatches => {
                    o.push(format!("{}:{}", pat.display(), matching_lines.len()));
                }
                DisplayMode::Regular => {
                    o.push(format!("{}", pat.display()));
                    for (line_num, lin) in matching_lines {
                        o.push(format!("{}:{}", line_num, lin));
                    }
                }
            }
        }
    }
    o
}

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} PATTERN [PATH] [options]", program);
    print!("{}", opts.usage(&brief));
}

fn opt_parser() -> Options {
    let mut opts = Options::new();
    opts.optflag(
        "l",
        "list-files",
        "List only files that contain the pattern",
    );
    opts.optflag(
        "c",
        "count",
        "Only print the number of matches in each file",
    );
    opts
}

pub fn get_opts(args: &[String]) -> Result<(String, String, Matches), String> {
    let program = args[0].clone();

    let opts = opt_parser();
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    match matches.free.clone().as_slice() {
        [] => {
            print_usage(&program, &opts);
            Err("not enough args".to_string())
        }
        [pattern] => Ok((pattern.to_string(), ".".to_string(), matches)),
        [pattern, path] => Ok((pattern.to_string(), path.to_string(), matches)),
        _ => {
            print_usage(&program, &opts);
            Err("too many args".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{display_output, get_opts};
    use std::path::Path;

    #[test]
    fn test_file_only_printer() {
        let p = Path::new("test_file.txt").to_path_buf();
        let m = vec![(1, "a match".to_string())];
        let args = vec!["self".to_string(), "beh".to_string(), "-l".to_string()];
        let (_, opts) = match get_opts(&args) {
            Ok((_, _, o)) => (1, o),
            Err(_) => panic!("at the disco"),
        };

        let file_result = vec![(p, m)];
        let output = display_output(file_result, &opts);
        assert_eq!(vec![format!("test_file.txt")], output);
    }

    #[test]
    fn test_regular_search_display() {
        let p = Path::new("test_file.txt").to_path_buf();
        let m = vec![(1, format!("a match"))];
        let args = vec![format!("self"), format!("beh")];
        let (_, opts) = match get_opts(&args) {
            Ok((_, _, o)) => (1, o),
            Err(_) => panic!("sure hope not"),
        };

        let file_result = vec![(p, m)];
        let output = display_output(file_result, &opts);
        assert_eq!(vec![format!("test_file.txt"), format!("1:a match")], output);
    }

    #[test]
    fn test_match_counter() {
        let p1 = Path::new("test_file.txt").to_path_buf();
        let m1 = vec![(1, "a match".to_string())];

        let p2 = Path::new("second_file.txt").to_path_buf();
        let m2 = vec![(1, "a match".to_string()), (2, "and another".to_string())];

        let args = vec!["self".to_string(), "beh".to_string(), "-c".to_string()];

        let (_, opts) = match get_opts(&args) {
            Ok((_, _, o)) => (1, o),
            Err(_) => panic!("should never happen"),
        };

        let file_result = vec![(p1, m1), (p2, m2)];
        let output = display_output(file_result, &opts);
        assert_eq!(
            vec![format!("test_file.txt:1"), format!("second_file.txt:2")],
            output
        );
    }

}
