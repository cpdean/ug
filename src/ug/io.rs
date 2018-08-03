use getopts::{Options, Matches};

use core;

enum DisplayMode {
    OnlyFiles,
    CountMatches,
    Regular // Wanted 'Default' but it seems to be a reserved word
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
    for (pat, linz) in results {
        if !linz.is_empty() {
            match get_display_mode(&opts) {
                DisplayMode::OnlyFiles => {
                    o.push(format!("{}", pat.display()));
                },
                DisplayMode::CountMatches => {
                    o.push(format!("{}:{}", pat.display(), linz.len()));
                },
                DisplayMode::Regular => {
                    o.push(format!("{}", pat.display()));
                    for (line_num, lin) in linz{
                        o.push(format!("{}:{}", line_num, lin));
                    }
                    o.push("".to_string());
                },
            }
        }
    }
    o
}


fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} PATTERN [options]", program);
    print!("{}", opts.usage(&brief));
}

fn opt_parser() -> Options {
    let mut opts = Options::new();
    opts.optflag("l", "list-files", "List only files that contain the pattern");
    opts.optflag("c", "count", "Only print the number of matches in each file");
    opts
}

pub fn get_opts(args: &[String]) -> Result<(String, Matches), String> {
    let program = args[0].clone();

    let opts = opt_parser();
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    let pattern = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, &opts);
        return Err("not enough args".to_string())
    };
    Ok((pattern, matches))

}

#[cfg(test)]
mod tests {
    use super::{get_opts, display_output};
    use std::path::{Path};

    #[test]
    fn test_file_only_printer() {
        let p = Path::new("test_file.txt").to_path_buf();
        let m = vec![(1, "a match".to_string())];
        let args = vec!["self".to_string(),
                        "beh".to_string(),
                        "-l".to_string()];
        let (_, opts) = match get_opts(args) {
            Ok((_, o)) => { (1, o) },
            Err(_) => { panic!("at the disco") },
        };

        let file_result = vec![(p, m)];
        let output = display_output(file_result, &opts);
        assert_eq!(vec![format!("test_file.txt")], output);
    }

    #[test]
    fn test_regular_search_display() {
        let p = Path::new("test_file.txt").to_path_buf();
        let m = vec![(1, format!("a match"))];
        let args = vec![format!("self"),
                        format!("beh")];
        let (_, opts) = match get_opts(args) {
            Ok((_, o)) => { (1, o) },
            Err(_) => { panic!("sure hope not") },
        };

        let file_result = vec![(p, m)];
        let output = display_output(file_result, &opts);
        assert_eq!(
            vec![
                format!("test_file.txt"),
                format!("1:a match"),
                format!(""),
            ],
            output
        );
    }

    #[test]
    fn test_match_counter() {
        let p1 = Path::new("test_file.txt").to_path_buf();
        let m1 = vec![(1, "a match".to_string())];

        let p2 = Path::new("second_file.txt").to_path_buf();
        let m2 = vec![
            (1, "a match".to_string()),
            (2, "and another".to_string())
        ];

        let args = vec!["self".to_string(),
                        "beh".to_string(),
                        "-c".to_string()];

        let (_, opts) = match get_opts(args) {
            Ok((_, o)) => { (1, o) },
            Err(_) => { panic!("should never happen") },
        };

        let file_result = vec![(p1, m1), (p2, m2)];
        let output = display_output(file_result, &opts);
        assert_eq!(
            vec![
                format!("test_file.txt:1"),
                format!("second_file.txt:2")
            ],
            output
        );
    }

}

