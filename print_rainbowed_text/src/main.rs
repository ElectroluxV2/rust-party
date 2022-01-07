use crate::Color::{Aqua, Blue, Green, Orange, Purple, Red, Yellow};

const RAINBOW: &'static [Color] = &[Red, Orange, Yellow, Green, Aqua, Blue, Purple];

fn main() {
    print_rainbow_text(&get_input_from_user());
}

fn get_input_from_user() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("stdin read_line()");
    input.trim().to_string()
}

fn print_rainbow_text(text: &str) {
    for current_color_index in 0..text.len() {
        RAINBOW.get(current_color_index % RAINBOW.len()).unwrap().write(&text.chars().nth(current_color_index).unwrap().to_string());
    }
    println!();
}

fn rgb_wrap(text: &str, r: u8, g: u8, b: u8) -> String {
    format!("\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, text)
}

#[derive(Debug)]
enum Color {
    // Navy,
    Blue,
    Aqua,
    // Teal,
    Purple,
    // Fuchsia,
    // Maroon,
    Red,
    Orange,
    Yellow,
    // Olive,
    Green,
    // Lime,
    // Black,
    // Gray,
    // Silver,
    // White
}

impl Color {
    fn value(&self) -> (u8, u8, u8) {
        match *self {
            // Color::Navy => (0x00, 0x1f, 0x3f),
            Color::Blue => (0x00, 0x7d, 0xd9),
            Color::Aqua => (0x7f, 0xdb, 0xff),
            // Color::Teal => (0x39, 0xcc, 0xcc),
            Color::Purple => (0xb1, 0x0d, 0xc9),
            // Color::Fuchsia => (0xf0, 0x12, 0xbe),
            // Color::Maroon => (0x85, 0x14, 0x4b),
            Color::Red => (0xff, 0x41, 0x36),
            Color::Orange => (0xff, 0x85, 0x1b),
            Color::Yellow => (0xff, 0xdc, 0x00),
            // Color::Olive => (0x3d, 0x99, 0x70),
            Color::Green => (0x2e, 0xcc, 0x40),
            // Color::Lime => (0x01, 0xff, 0x70),
            // Color::Black => (0x11, 0x11, 0x11),
            // Color::Gray => (0xaa, 0xaa, 0xaa),
            // Color::Silver => (0xdd, 0xdd, 0xdd),
            // Color::White => (0xff, 0xff, 0xff)
        }
    }

    fn write(&self, text: &str) {
        print!("\x1b[38;2;{};{};{}m{}\x1b[0m", self.value().0, self.value().1, self.value().2, text);
    }
}