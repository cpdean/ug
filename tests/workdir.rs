/*

taken from ripgrep's test suite

https://github.com/BurntSushi/ripgrep/blob/2913fc4cd063f4d869f54497a313aafbf5330346/tests/workdir.rs

original license:

The MIT License (MIT)

Copyright (c) 2015 Andrew Gallant

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.

*/

use std::env;
use std::error;
use std::fmt;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process;
use std::str::FromStr;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
use std::thread;
use std::time::Duration;

static TEST_DIR: &'static str = "the-integration-tests";
static NEXT_ID: AtomicUsize = ATOMIC_USIZE_INIT;

/// `WorkDir` represents a directory in which tests are run.
///
/// Directories are created from a global atomic counter to avoid duplicates.
#[derive(Debug)]
pub struct WorkDir {
    /// The directory in which this test executable is running.
    root: PathBuf,
    /// The directory in which the test should run. If a test needs to create
    /// files, they should go in here.
    dir: PathBuf,
}

impl WorkDir {
    /// Create a new test working directory with the given name. The name
    /// does not need to be distinct for each invocation, but should correspond
    /// to a logical grouping of tests.
    pub fn new(name: &str) -> WorkDir {
        let id = NEXT_ID.fetch_add(1, Ordering::SeqCst);
        let root = env::current_exe()
            .unwrap()
            .parent()
            .expect("executable's directory")
            .to_path_buf();
        let dir = root.join(TEST_DIR).join(name).join(&format!("{}", id));
        nice_err(&dir, repeat(|| fs::create_dir_all(&dir)));
        WorkDir {
            root: root,
            dir: dir,
        }
    }

    /// Create a new file with the given name and contents in this directory,
    /// or panic on error.
    pub fn create<P: AsRef<Path>>(&self, name: P, contents: &str) {
        self.create_bytes(name, contents.as_bytes());
    }

    /// Try to create a new file with the given name and contents in this
    /// directory.
    pub fn try_create<P: AsRef<Path>>(&self, name: P, contents: &str) -> io::Result<()> {
        let path = self.dir.join(name);
        self.try_create_bytes(path, contents.as_bytes())
    }

    /// Create a new file with the given name and size.
    pub fn create_size<P: AsRef<Path>>(&self, name: P, filesize: u64) {
        let path = self.dir.join(name);
        let file = nice_err(&path, File::create(&path));
        nice_err(&path, file.set_len(filesize));
    }

    /// Create a new file with the given name and contents in this directory,
    /// or panic on error.
    pub fn create_bytes<P: AsRef<Path>>(&self, name: P, contents: &[u8]) {
        let path = self.dir.join(name);
        nice_err(&path, self.try_create_bytes(&path, contents));
    }

    /// Try to create a new file with the given name and contents in this
    /// directory.
    fn try_create_bytes<P: AsRef<Path>>(&self, path: P, contents: &[u8]) -> io::Result<()> {
        let mut file = File::create(&path)?;
        file.write_all(contents)?;
        file.flush()
    }

    /// Remove a file with the given name from this directory.
    pub fn remove<P: AsRef<Path>>(&self, name: P) {
        let path = self.dir.join(name);
        nice_err(&path, fs::remove_file(&path));
    }

    /// Create a new directory with the given path (and any directories above
    /// it) inside this directory.
    pub fn create_dir<P: AsRef<Path>>(&self, path: P) {
        let path = self.dir.join(path);
        nice_err(&path, repeat(|| fs::create_dir_all(&path)));
    }

    /// Creates a new command that is set to use the ripgrep executable in
    /// this working directory.
    pub fn command(&self) -> process::Command {
        let mut cmd = process::Command::new(&self.bin());
        cmd.env_remove("RIPGREP_CONFIG_PATH");
        cmd.current_dir(&self.dir);
        cmd
    }

    /// Returns the path to the ripgrep executable.
    #[cfg(not(windows))]
    pub fn bin(&self) -> PathBuf {
        let path = self.root.join("ug");
        if !path.is_file() {
            // Looks like a recent version of Cargo changed the cwd or the
            // location of the test executable.
            self.root.join("../ug")
        } else {
            path
        }
    }

    /// Returns the path to this directory.
    pub fn path(&self) -> &Path {
        &self.dir
    }

    /// Creates a directory symlink to the src with the given target name
    /// in this directory.
    #[cfg(not(windows))]
    pub fn link_dir<S: AsRef<Path>, T: AsRef<Path>>(&self, src: S, target: T) {
        use std::os::unix::fs::symlink;
        let src = self.dir.join(src);
        let target = self.dir.join(target);
        let _ = fs::remove_file(&target);
        nice_err(&target, symlink(&src, &target));
    }

    /// Creates a directory symlink to the src with the given target name
    /// in this directory.
    #[cfg(windows)]
    pub fn link_dir<S: AsRef<Path>, T: AsRef<Path>>(&self, src: S, target: T) {
        use std::os::windows::fs::symlink_dir;
        let src = self.dir.join(src);
        let target = self.dir.join(target);
        let _ = fs::remove_dir(&target);
        nice_err(&target, symlink_dir(&src, &target));
    }

    /// Creates a file symlink to the src with the given target name
    /// in this directory.
    #[cfg(not(windows))]
    pub fn link_file<S: AsRef<Path>, T: AsRef<Path>>(&self, src: S, target: T) {
        self.link_dir(src, target);
    }

    /// Creates a file symlink to the src with the given target name
    /// in this directory.
    #[cfg(windows)]
    pub fn link_file<S: AsRef<Path>, T: AsRef<Path>>(&self, src: S, target: T) {
        use std::os::windows::fs::symlink_file;
        let src = self.dir.join(src);
        let target = self.dir.join(target);
        let _ = fs::remove_file(&target);
        nice_err(&target, symlink_file(&src, &target));
    }

    /// Runs and captures the stdout of the given command.
    ///
    /// If the return type could not be created from a string, then this
    /// panics.
    pub fn stdout<E: fmt::Debug, T: FromStr<Err = E>>(&self, cmd: &mut process::Command) -> T {
        let o = self.output(cmd);
        let stdout = String::from_utf8_lossy(&o.stdout);
        match stdout.parse() {
            Ok(t) => t,
            Err(err) => {
                panic!("could not convert from string: {:?}\n\n{}", err, stdout);
            }
        }
    }

    /// Gets the output of a command. If the command failed, then this panics.
    pub fn output(&self, cmd: &mut process::Command) -> process::Output {
        let output = cmd.output().unwrap();
        self.expect_success(cmd, output)
    }

    /// Pipe `input` to a command, and collect the output.
    pub fn pipe(&self, cmd: &mut process::Command, input: &str) -> process::Output {
        cmd.stdin(process::Stdio::piped());
        cmd.stdout(process::Stdio::piped());
        cmd.stderr(process::Stdio::piped());

        let mut child = cmd.spawn().unwrap();

        // Pipe input to child process using a separate thread to avoid
        // risk of deadlock between parent and child process.
        let mut stdin = child.stdin.take().expect("expected standard input");
        let input = input.to_owned();
        let worker = thread::spawn(move || write!(stdin, "{}", input));

        let output = self.expect_success(cmd, child.wait_with_output().unwrap());
        worker.join().unwrap().unwrap();
        output
    }

    /// If `o` is not the output of a successful process run
    fn expect_success(&self, cmd: &process::Command, o: process::Output) -> process::Output {
        if !o.status.success() {
            let suggest = if o.stderr.is_empty() {
                "\n\nDid your search end up with no results?".to_string()
            } else {
                "".to_string()
            };

            panic!(
                "\n\n==========\n\
                 command failed but expected success!\
                 {}\
                 \n\ncommand: {:?}\
                 \ncwd: {}\
                 \n\nstatus: {}\
                 \n\nstdout: {}\
                 \n\nstderr: {}\
                 \n\n==========\n",
                suggest,
                cmd,
                self.dir.display(),
                o.status,
                String::from_utf8_lossy(&o.stdout),
                String::from_utf8_lossy(&o.stderr)
            );
        }
        o
    }

    /// Runs the given command and asserts that it resulted in an error exit
    /// code.
    pub fn assert_err(&self, cmd: &mut process::Command) {
        let o = cmd.output().unwrap();
        if o.status.success() {
            panic!(
                "\n\n===== {:?} =====\n\
                 command succeeded but expected failure!\
                 \n\ncwd: {}\
                 \n\nstatus: {}\
                 \n\nstdout: {}\n\nstderr: {}\
                 \n\n=====\n",
                cmd,
                self.dir.display(),
                o.status,
                String::from_utf8_lossy(&o.stdout),
                String::from_utf8_lossy(&o.stderr)
            );
        }
    }

    /// Runs the given command and asserts that its exit code matches expected exit code.
    pub fn assert_exit_code(&self, expected_code: i32, cmd: &mut process::Command) {
        let code = cmd.status().unwrap().code().unwrap();

        assert_eq!(
            expected_code, code,
            "\n\n===== {:?} =====\n\
             expected exit code did not match\
             \n\nexpected: {}\
             \n\nfound: {}\
             \n\n=====\n",
            cmd, expected_code, code
        );
    }

    /// Runs the given command and asserts that something was printed to
    /// stderr.
    pub fn assert_non_empty_stderr(&self, cmd: &mut process::Command) {
        let o = cmd.output().unwrap();
        if o.status.success() || o.stderr.is_empty() {
            panic!(
                "\n\n===== {:?} =====\n\
                 command succeeded but expected failure!\
                 \n\ncwd: {}\
                 \n\nstatus: {}\
                 \n\nstdout: {}\n\nstderr: {}\
                 \n\n=====\n",
                cmd,
                self.dir.display(),
                o.status,
                String::from_utf8_lossy(&o.stdout),
                String::from_utf8_lossy(&o.stderr)
            );
        }
    }
}

fn nice_err<P: AsRef<Path>, T, E: error::Error>(path: P, res: Result<T, E>) -> T {
    match res {
        Ok(t) => t,
        Err(err) => {
            panic!("{}: {:?}", path.as_ref().display(), err);
        }
    }
}

fn repeat<F: FnMut() -> io::Result<()>>(mut f: F) -> io::Result<()> {
    let mut last_err = None;
    for _ in 0..10 {
        if let Err(err) = f() {
            last_err = Some(err);
            thread::sleep(Duration::from_millis(500));
        } else {
            return Ok(());
        }
    }
    Err(last_err.unwrap())
}
