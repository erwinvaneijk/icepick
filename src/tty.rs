#![allow(unstable)]

extern crate libc;

use winsize;

use std::io::{File, Open, ReadWrite, Command};
use std::io::process::StdioContainer;
use std::os::unix::prelude::AsRawFd;

pub struct TTY {
    file: File,
    dimensions: (usize, usize),
    original_state: String
}

pub trait IO {
    fn write(&mut self, line: &str);
    fn read(&mut self) -> Option<String>;
    fn last(&self) -> &str;
    fn lines(&self) -> Vec<String>;
    fn dimensions(&self) -> (usize, usize);
}

impl IO for TTY {
    fn write(&mut self, line: &str) {
        let it = format!("{}\n", line);
        self.file.write_str(it.as_slice());
    }

    fn read(&mut self) -> Option<String> {
        let res = match self.file.read_byte() {
            Ok(c) => {
                let character = c as char;
                Some(character.to_string())
            },
            Err(_) => None,
        };
        res
    }

    fn last(&self) -> &str {
        "fail"
    }

    fn lines(&self) -> Vec<String> {
        let mut lines: Vec<String> = Vec::new();
        lines.push("fail".to_string());
        lines
    }

    fn dimensions(&self) -> (usize, usize) {
        self.dimensions
    }
}

impl TTY {
    pub fn new() -> TTY {
        let path = Path::new("/dev/tty");
        let file = File::open_mode(&path, Open, ReadWrite).unwrap();
        TTY::no_echo_no_escaping(&file);

        TTY {
            original_state: TTY::previous_state(&file),
            dimensions: winsize::get_winsize(&file).unwrap(),
            file: file,
        }
    }

    fn stty(file: &File, args: &[&str]) -> Option<String> {
        let container = StdioContainer::InheritFd(file.as_raw_fd());
        let output = Command::new("stty").args(args).stdin(container).output().unwrap();
        String::from_utf8(output.output).ok()
    }

    fn no_echo_no_escaping(file: &File) {
        TTY::stty(file, &["-echo", "-icanon"]);
    }

    fn previous_state(file: &File) -> String {
        TTY::stty(file, &["-g"]).unwrap_or("".to_string())
    }

    fn reset(self) {
        TTY::stty(&self.file, &[self.original_state.as_slice()]);
    }
}

#[cfg(test)]

#[test]
fn can_create_a_tty() {
    let mut tty = TTY::new();
    tty.write("##### a string        \n");
    //tty.read();
}

#[test]
fn winsize_has_valid_width_and_height() {
    let tty = TTY::new();
    let (width, height) = tty.dimensions;
    assert!(width > 0);
    assert!(height > 0);
    tty.reset();
}
