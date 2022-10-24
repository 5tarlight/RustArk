use colored::Colorize;
use rust_ark::console::*;

#[test]
fn print_red() {
    let red = format("Hello", ConsoleColor::Red);
    assert_eq!(red, format!("Hello").red());
}

#[test]
fn print_black() {
    let black = format("Hello", ConsoleColor::Black);
    assert_eq!(black, format!("Hello").black());
}
#[test]
fn print_blue() {
    let blue = format("Hello", ConsoleColor::Blue);
    assert_eq!(blue, format!("Hello").blue());
}
#[test]
fn print_green() {
    let green = format("Hello", ConsoleColor::Green);
    assert_eq!(green, format!("Hello").green());
}
#[test]
fn print_purple() {
    let purple = format("Hello", ConsoleColor::Purple);
    assert_eq!(purple, format!("Hello").purple());
}

#[test]
fn print_yellow() {
    let yellow = format("Hello", ConsoleColor::Yellow);
    assert_eq!(yellow, format!("Hello").yellow());
}

#[test]
fn print_underline() {
    let underline = format_deco("Hello", ConsoleColor::Black, ConsoleDecoration::Underline);
    assert_eq!(underline, format!("Hello").black().underline())
}

#[test]
fn print_italic() {
    let italic = format_deco("Hello", ConsoleColor::Black, ConsoleDecoration::Italic);
    assert_eq!(italic, format!("Hello").black().italic())
}
#[test]
fn print_strikethrougn() {
    let strike_through = format_deco(
        "Hello",
        ConsoleColor::Black,
        ConsoleDecoration::StrikeThrougb,
    );
    assert_eq!(strike_through, format!("Hello").black().strikethrough());
}
