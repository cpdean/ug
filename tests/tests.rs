/*!
This module contains *integration* tests. Their purpose is to test the CLI
interface. Namely, that passing a flag does what it says on the tin.

Tests for more fine grained behavior (like the search or the globber) should be
unit tests in their respective modules.
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
