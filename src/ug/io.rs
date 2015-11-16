use getopts::{Options, Matches};

use core;

/// given the matches, generate output as a
/// stream of lines that will then be printed later
pub fn display_output(results: Vec<core::FileResult>, opts: &Matches) -> Vec<String> {
    let mut o: Vec<String> = Vec::new();
    for (pat, linz) in results {
        if !linz.is_empty() {
            o.push(format!("{}", pat.display()));
            if !opts.opt_present("l") {
                for (line_num, lin) in linz{
                    o.push(format!("{}:{}", line_num, lin));
                }
                o.push(format!(""));
            }
        }
    }
    o
}


fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} PATTERN [options]", program);
    print!("{}", opts.usage(&brief));
}

pub fn get_opts(args: Vec<String>) -> Result<(String, Matches), String> {
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

