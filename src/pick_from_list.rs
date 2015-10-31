extern crate std;

use std::io;
use std::io::{Read, Write, BufReader, BufRead};
use std::fs::{File, OpenOptions};
use std::process::{Command, Stdio};
use std::str::FromStr;
use std::env;

fn pick_from_list_external(cmd: &mut Command, items: &[&str]) -> io::Result<String> {
    let process = try!(cmd.stdin(Stdio::piped()).stdout(Stdio::piped()).spawn());
    {
        let mut process_in = process.stdin.unwrap();
        for item in items {
            try!(process_in.write_all((item.replace("\n", "") + "\n").as_bytes()))
        }
    }
    let mut result = String::new();
    try!(process.stdout.unwrap().read_to_string(&mut result));
    Ok(result.replace("\n", ""))
}

fn read_parse<T>(tty: &mut File, prompt: &str, min: T, max: T) -> io::Result<T> where T: FromStr + Ord {
    try!(tty.write_all(prompt.as_bytes()));
    let mut reader = BufReader::new(tty);
    let mut result = String::new();
    try!(reader.read_line(&mut result));
    match result.replace("\n", "").parse::<T>() {
        Ok(x) => if x >= min && x <= max { Ok(x) } else { read_parse(reader.into_inner(), prompt, min, max) },
        _ => read_parse(reader.into_inner(), prompt, min, max)
    }
}

fn pick_from_list_internal(items: &[&str], prompt: &str) -> io::Result<String> {
    let mut tty = try!(OpenOptions::new().read(true).write(true).open("/dev/tty"));
    let pad_len = ((items.len() as f32).log10().floor() + 1.0) as usize;
    for (i, item) in items.iter().enumerate() {
        try!(tty.write_all(format!("{1:0$}. {2}\n", pad_len, i + 1, item.replace("\n", "")).as_bytes()))
    }
    let idx = try!(read_parse::<usize>(&mut tty, prompt, 1, items.len())) - 1;
    Ok(items[idx].to_string())
}

/// Asks the user to select an item from a list.
///
/// If `cmd` is `Some`, an external menu program will be used.  
/// Otherwise, a built-in simple number-based command-line menu (on `/dev/tty`) will be used, with a `prompt`.
///
/// Note: an external program might return something that's not in the list!
pub fn pick_from_list(cmd: Option<&mut Command>, items: &[&str], prompt: &str) -> io::Result<String> {
    match cmd {
        Some(command) => pick_from_list_external(command, items),
        None => pick_from_list_internal(items, prompt),
    }
}

/// Returns the user's preferred menu program from the `MENU` environment variable if it exists.
/// 
/// Use `.as_mut()` on the returned value to turn in into an `Option<&mut Command>`.
pub fn default_menu_cmd() -> Option<Command> {
    match env::var("MENU") {
        Ok(val) => Some(Command::new(val)),
        Err(_) => None
    }
}
