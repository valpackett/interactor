extern crate std;
extern crate termios;

use std::os::unix::io::AsRawFd;
use std::io::{Read, Write};
use std::fs::{File, OpenOptions};
use self::termios::*;

pub fn read_from_tty_file<F>(tty: &mut File, byte_callback: F, run_before_input: bool, run_after_input: bool) -> std::io::Result<Vec<u8>>
    where F : Fn(&[u8], u8, &mut File) {
    let fd = tty.as_raw_fd();
    let mut termios = try!(Termios::from_fd(fd));
    let orig_lflag = termios.c_lflag;
    termios.c_lflag &= !(ECHO | ICANON);
    try!(tcsetattr(fd, TCSANOW, &termios));

    let mut buffer = [0; 1];
    let mut result = Vec::new();
    if run_before_input {
        byte_callback(&result, buffer[0], tty);
    }
    loop {
        try!(tty.read(&mut buffer));
        match buffer[0] {
            10 | 13 => { break }
            8 | 127 => { result.pop(); }
            0 ... 31 => {}
            _ => result.push(buffer[0])
        };
        byte_callback(&result, buffer[0], tty);
    }
    if run_after_input {
        byte_callback(&result, 4, tty);
    }

    termios.c_lflag = orig_lflag;
    try!(tcsetattr(fd, TCSANOW, &termios));

    Ok(result)
}

pub fn read_from_tty<F>(byte_callback: F, run_before_input: bool, run_after_input: bool) -> std::io::Result<Vec<u8>>
    where F : Fn(&[u8], u8, &mut File) {
    let mut tty = try!(OpenOptions::new().read(true).write(true).open("/dev/tty"));
    read_from_tty_file(&mut tty, byte_callback, run_before_input, run_after_input)
}
