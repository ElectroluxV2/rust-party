use crate::Color::{Blue, Green, Indigo, Orange, Red, Violet, Yellow};
use std::io;

enum Color {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Indigo,
    Violet,
}

impl Color {
    fn value(&self) -> (u8, u8, u8) {
        match *self {
            Color::Red => (255, 65, 54),
            Color::Orange => (255, 140, 0),
            Color::Yellow => (255, 255, 0),
            Color::Green => (0, 255, 0),
            Color::Blue => (0, 191, 255),
            Color::Indigo => (75, 0, 130),
            Color::Violet => (148, 0, 211),
        }
    }
    fn write(&self, text: &str) -> String {
        format!(
            "\x1b[38;2;{};{};{}m{}\x1b[0m",
            self.value().0,
            self.value().1,
            self.value().2,
            text
        )
    }
}

fn main() {
    let input = get_input_from_user();
    print_rainbowed_text(&input);
}

fn get_input_from_user() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");
    input.trim().to_string()
}

fn print_rainbowed_text(text: &str) {
    let array = [Red, Orange, Yellow, Green, Blue, Indigo, Violet];
    for i in 0..text.chars().count() {
        print!(
            "{}",
            array
                .get(i % 7)
                .unwrap()
                .write(&text.chars().nth(i).unwrap().to_string())
        )
    }
}
