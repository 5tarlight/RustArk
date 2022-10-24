use lib::console::{printc, printlnc};

pub mod lib;

fn main() {
    printc("Welcome to ", lib::console::ConsoleColor::Blue);
    printlnc("Rust Ark", lib::console::ConsoleColor::Purple);

    let termsize::Size { rows, cols } = termsize::get().unwrap();
    println!("          width  height");
    println!("termsize: {:4}    {:4}", cols, rows);
}
