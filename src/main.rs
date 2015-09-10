extern crate interactor;
use interactor::*;
use std::process::Command;

fn main() {
    println!("Interactor demo.");

    let chosen_ext = pick_from_list(Some(&mut Command::new("peco")), &["first", "second"], "").unwrap();
    println!("Congratulations, you chose '{}'!!", chosen_ext);

    let chosen_int = pick_from_list(None, &["first", "second"], "Selection: ").unwrap();
    println!("Congratulations, you chose '{}'!!", chosen_int);
}
