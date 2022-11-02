use rust_ark::console::{clear, estimate_size, window::Window};

#[cfg(test)]
mod tests;

/// Main function.
/// Start rust-ark process.
/// This application is based on console and cross-platform.
/// In some terminals, colored text may not be displayed properly.
fn main() {
    clear();
    // printc("Welcome to ", ConsoleColor::Blue);
    // printlnc("Rust Ark", ConsoleColor::Purple);

    let size = estimate_size();
    let mut window = Window::new(size.0, size.1);

    window.add_tab(String::from("Main"));
    window.add_tab(String::from("Inventory"));
    window.show();
}
