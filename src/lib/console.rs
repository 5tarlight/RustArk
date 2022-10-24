use colored::{ColoredString, Colorize};

#[allow(dead_code)]
pub enum ConsoleColor {
    Black,
    Red,
    Yellow,
    Green,
    Blue,
    Purple,
}

#[allow(dead_code)]
pub enum ConsoleDecoration {
    Underline,
    StrikeThrougb,
    Italic,
    None,
}

#[allow(dead_code)]
pub fn format(txt: &'static str, color: ConsoleColor) -> ColoredString {
    match color {
        ConsoleColor::Black => format!("{txt}").black(),
        ConsoleColor::Red => format!("{txt}").red(),
        ConsoleColor::Yellow => format!("{txt}").yellow(),
        ConsoleColor::Green => format!("{txt}").green(),
        ConsoleColor::Blue => format!("{txt}").blue(),
        ConsoleColor::Purple => format!("{txt}").purple(),
    }
}

#[allow(dead_code)]
pub fn format_deco(
    txt: &'static str,
    color: ConsoleColor,
    back: ConsoleDecoration,
) -> ColoredString {
    let coloured = format(txt, color);

    match back {
        ConsoleDecoration::Underline => coloured.underline(),
        ConsoleDecoration::StrikeThrougb => coloured.strikethrough(),
        ConsoleDecoration::Italic => coloured.italic(),
        ConsoleDecoration::None => coloured,
    }
}

#[allow(dead_code)]
pub fn printc(txt: &'static str, color: ConsoleColor) {
    print!("{}", format(txt, color));
}

#[allow(dead_code)]
pub fn printlnc(txt: &'static str, color: ConsoleColor) {
    println!("{}", format(txt, color));
}

#[allow(dead_code)]
pub fn print_deco(txt: &'static str, color: ConsoleColor, back: ConsoleDecoration) {
    print!("{}", format_deco(txt, color, back))
}

#[allow(dead_code)]
pub fn println_deco(txt: &'static str, color: ConsoleColor, back: ConsoleDecoration) {
    println!("{}", format_deco(txt, color, back))
}
