/*

repurposed from ripgrep's integration framework
https://github.com/BurntSushi/ripgrep/blob/2913fc4cd063f4d869f54497a313aafbf5330346/tests/tests.rs

*/

#![allow(dead_code, unused_imports)]

use std::process::Command;

use workdir::WorkDir;

mod workdir;

macro_rules! clean {
    ($name:ident, $query:expr, $path:expr, $fun:expr) => {
        #[test]
        fn $name() {
            let wd = WorkDir::new(stringify!($name));
            // until https://github.com/cpdean/ug/issues/6
            wd.create(".gitignore", "");
            let mut cmd = wd.command();
            cmd.arg($query).arg($path);
            $fun(wd, cmd);
        }
    };
}

fn path(unix: &str) -> String {
    if cfg!(windows) {
        unix.replace("/", "\\")
    } else {
        unix.to_string()
    }
}

fn paths(unix: &[&str]) -> Vec<String> {
    let mut xs: Vec<_> = unix.iter().map(|s| path(s)).collect();
    xs.sort();
    xs
}

fn paths_from_stdout(stdout: String) -> Vec<String> {
    let mut paths: Vec<_> = stdout.lines().map(|s| {
        s.split(':').next().unwrap().to_string()
    }).collect();
    paths.sort();
    paths
}

fn sort_lines(lines: &str) -> String {
    let mut lines: Vec<String> =
        lines.trim().lines().map(|s| s.to_owned()).collect();
    lines.sort();
    format!("{}\n", lines.join("\n"))
}

fn cmd_exists(name: &str) -> bool {
    Command::new(name).arg("--help").output().is_ok()
}

clean!(please_goat, "goat", ".", |wd: WorkDir, mut cmd: Command| {
    wd.create("foo", "goat");

    let lines: String = wd.stdout(&mut cmd);
    assert_eq!(lines, "./foo\n1:goat\n");
});

clean!(ignore_default_gitdir, "test", ".", |wd: WorkDir, mut cmd: Command| {
    wd.create_dir(".git");
    wd.create(".git/foo", "test");
    wd.create("foo", "test");

    let lines: String = wd.stdout(&mut cmd);
    assert_eq!(lines, "./foo\n1:test\n");
});

clean!(ignore_file_by_git_ignore, "test", ".", |wd: WorkDir, mut cmd: Command| {
    wd.create(".gitignore", "ignore_me");
    wd.create("ignore_me", "test");
    wd.create("foo", "test");

    let lines: String = wd.stdout(&mut cmd);
    assert_eq!(lines, "./foo\n1:test\n");
});

#[test]
fn test_empty_dir() {
    let wd = WorkDir::new("empty_dir");
    // until https://github.com/cpdean/ug/issues/6
    wd.create(".gitignore", "");

    let mut cmd = wd.command();
    cmd.arg(".");
    let lines: String = wd.stdout(&mut cmd);
    assert_eq!(lines, "");
}
