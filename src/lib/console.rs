use colored::{ColoredString, Colorize};

pub enum ConsoleColor {
    Black,
    Red,
    Yellow,
    Green,
    Blue,
    Purple,
}

pub enum ConsoleDecoration {
    Underline,
    StrikeThrougb,
    Italic,
    None,
}

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

pub fn printc(txt: &'static str, color: ConsoleColor) {
    print!("{}", format(txt, color));
}

pub fn printlnc(txt: &'static str, color: ConsoleColor) {
    println!("{}", format(txt, color));
}

pub fn print_deco(txt: &'static str, color: ConsoleColor, back: ConsoleDecoration) {
    print!("{}", format_deco(txt, color, back))
}

pub fn println_deco(txt: &'static str, color: ConsoleColor, back: ConsoleDecoration) {
    println!("{}", format_deco(txt, color, back))
}
