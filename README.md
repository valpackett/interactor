# interactor [![crates.io](https://img.shields.io/crates/v/interactor.svg)](https://crates.io/crates/interactor) [![Build Status](https://img.shields.io/travis/myfreeweb/interactor.svg?style=flat)](https://travis-ci.org/myfreeweb/interactor) [![API Docs](https://img.shields.io/badge/api-docs-yellow.svg?style=flat)](https://myfreeweb.github.io/autodocs/interactor/interactor) [![unlicense](https://img.shields.io/badge/un-license-green.svg?style=flat)](http://unlicense.org)

A [Rust] library for simple (usually command-line) user interaction.

- Reading input from the console with a callback for each byte (e.g. for displaying [colorhash256] for a password)
- Selecting an item from a list using an external menu program (usually a fuzzy finder) or a built-in simple menu
- Selecting a file using the "item from a list" thing above

[Rust]: https://www.rust-lang.org
[colorhash256]: https://github.com/myfreeweb/colorhash256


## Menu program?

A program that accepts a newline-separated list of items on `stdin`, presents a UI to the user (directly on `/dev/tty` if it's a CLI; can also be a GUI), and outputs the selected item on `stdout`.

- [fzf](https://github.com/junegunn/fzf) (Go)
- [peco](https://github.com/peco/peco) (Go)
- [percol](https://github.com/mooz/percol) (Python)
- [icepick](https://github.com/felipesere/icepick) (Rust)
- [heatseeker](https://github.com/rschmitt/heatseeker) (Rust)
- [selecta](https://github.com/garybernhardt/selecta) (Ruby)
- [hf](https://github.com/Refefer/hf) (Haskell)
- [dmenu](http://tools.suckless.org/dmenu/) (C, **X11 GUI**)
- [rofi](https://github.com/DaveDavenport/rofi) (C, **X11 GUI**)

You should let the users of your application pick their own favorite tool as a config option.  
I propose the `$MENU` environment variable, like `$EDITOR`, as a place to look for user preference.

## Usage

```rust
extern crate interactor;
use interactor::*;

fn main() {
    let read_result = read_from_tty(|buf, b, tty| {
        tty.write(&format!("({:?} | {})\n", buf, b).into_bytes());
    }, false, false).unwrap();
    println!("Read: {}", String::from_utf8(read_result).unwrap());

    let chosen_ext = pick_from_list(default_menu_cmd().as_mut(), &["first", "second"], "Selection: ").unwrap();
    println!("Congratulations, you chose '{}'!!", chosen_ext);
}
```

## Contributing

Please feel free to submit pull requests!

By participating in this project you agree to follow the [Contributor Code of Conduct](http://contributor-covenant.org/version/1/4/).

[The list of contributors is available on GitHub](https://github.com/myfreeweb/interactor/graphs/contributors).

## License

This is free and unencumbered software released into the public domain.  
For more information, please refer to the `UNLICENSE` file or [unlicense.org](http://unlicense.org).
