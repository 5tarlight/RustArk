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
    window.tabs[0].set_content(String::from("1\n2\n3\n4\n5\n6\n7\n8\n9\n0\n1\n2\n3\n4\n5\n6\n7\n8\n9\n0\n1\n2\n3\n4\n5\n6\n7\n8\n9\n0\n"));
    window.add_tab(String::from("Inventory"));
    window.tabs[1].set_content(String::from("Hello World from inventory"));
    window.show();
}
