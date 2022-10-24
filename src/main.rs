use rust_ark::console::{clear, estimate_size, printc, printlnc, ConsoleColor};

fn main() {
    clear();
    printc("Welcome to ", ConsoleColor::Blue);
    printlnc("Rust Ark", ConsoleColor::Purple);

    println!("{:?}", estimate_size());
}
