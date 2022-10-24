use lib::console::{printc, printlnc};

pub mod lib;

fn main() {
    printc("Welcome to ", lib::console::ConsoleColor::Blue);
    printlnc("Rust Ark", lib::console::ConsoleColor::Purple);
}
