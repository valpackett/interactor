extern crate interactor;
use interactor::*;

fn main() {
    println!("Interactor demo.");

    let chosen_ext = pick_from_list(default_menu_cmd().as_mut(), &["first", "second"], "").unwrap();
    println!("Congratulations, you chose '{}'!!", chosen_ext);

    let chosen_int = pick_from_list(None, &["first", "second"], "Selection: ").unwrap();
    println!("Congratulations, you chose '{}'!!", chosen_int);
}
