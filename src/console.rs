use core::panic;
use std::io;
use std::io::prelude::*;

use colored::{ColoredString, Colorize};

/// Color of colored text to be printed to console
pub enum ConsoleColor {
    Black,
    Red,
    Yellow,
    Green,
    Blue,
    Purple,
}

/// Decoration of text.
/// Mixture of multiple decoration is not supported by console.
pub enum ConsoleDecoration {
    Underline,
    StrikeThrougb,
    Italic,
    None,
}

/// format `String` literal to `ColoredString`.
///
/// ```
/// use rust_ark::console::*;
/// use colored::Colorize;
///
/// let red = format("Hello", ConsoleColor::Red);
/// assert_eq!(red, format!("Hello").red());
/// ```
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

/// format `String` literal to colored and decorated `ColoredString`.
///
/// ```
/// use rust_ark::console::*;
/// use colored::Colorize;
///
/// let red = format_deco("Hello", ConsoleColor::Red, ConsoleDecoration::Underline);
/// assert_eq!(red, format!("Hello").red().underline());
/// ```
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

/// Print `ColoredText` without new line.
/// ```
/// use rust_ark::console::*;
/// printc("Hello", ConsoleColor::Black);
/// ```
pub fn printc(txt: &'static str, color: ConsoleColor) {
    print!("{}", format(txt, color));
}

/// Print `ColoredText` with new line.
/// ```
/// use rust_ark::console::*;
/// printlnc("Hello", ConsoleColor::Black);
/// ```
pub fn printlnc(txt: &'static str, color: ConsoleColor) {
    println!("{}", format(txt, color));
}

/// Print `ColoredText` with decoration and no new line.
/// ```
/// use rust_ark::console::*;
/// print_deco("Hello", ConsoleColor::Black, ConsoleDecoration::Underline);
/// ```
pub fn print_deco(txt: &'static str, color: ConsoleColor, back: ConsoleDecoration) {
    print!("{}", format_deco(txt, color, back))
}

/// Print `ColoredText` with decoration and new line.
/// ```
/// use rust_ark::console::*;
/// println_deco("Hello", ConsoleColor::Black, ConsoleDecoration::Underline);
/// ```
pub fn println_deco(txt: &'static str, color: ConsoleColor, back: ConsoleDecoration) {
    println!("{}", format_deco(txt, color, back))
}

/// Clear console and move cursor to first index.
/// In some environment, this may be broken.
pub fn clear() {
    print!("\x1B[2J\x1B[1;1H");
}

/// Pause the process and wait for next key press.
/// **This function freezes current thread**
pub fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "\n\n아무 키를 눌러 계속하세요...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

/// Get width and height of current console.
/// Unit is word.
/// Recommend using programming fonts (All widths of letter is same.)
///
/// # Panic
/// If terminal do not support reading width and height of itself, This method will be paniced.
pub fn estimate_size() -> (u16, u16) {
    let termsize::Size {
        rows: height,
        cols: width,
    } = match termsize::get() {
        Some(r) => r,
        None => {
            eprintln!("콘솔의 가로와 세로 사이즈를 계산할 수 없습니다.");
            panic!("이 터미널이 필요 기능을 지원하지 않습니다.")
        }
    };

    if width < 128 {
        println_deco(
            "가로 길이가 너무 짧습니다.",
            ConsoleColor::Red,
            ConsoleDecoration::Underline,
        );
        pause();
    } else if height < 32 {
        println_deco(
            "세로 길이가 너무 짧습니다.",
            ConsoleColor::Red,
            ConsoleDecoration::Underline,
        );
        pause();
    }

    (width, height)
}
