use rust_ark::console::{clear, estimate_size, printc, printlnc, ConsoleColor};

/// Main function.
/// Start rust-ark process.
/// This application is based on console and cross-platform.
/// In some terminals, colored text may not be displayed properly.
fn main() {
    clear();
    printc("Welcome to ", ConsoleColor::Blue);
    printlnc("Rust Ark", ConsoleColor::Purple);

    println!("{:?}", estimate_size());
}
