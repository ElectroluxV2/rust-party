
fn main() {
    let input = get_input_from_user();
    print_rainbow_text(&input);
    test_red_color();
}

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
            Color::Red => (0xFF, 0x41, 0x36),
            Color::Orange => (0xFF, 0x85, 0x1B),
            Color::Yellow => (0xFF, 0xDC, 0x00),
            Color::Green => (0x2E, 0xCC, 0x40),
            Color::Blue => (0x00, 0x74, 0xD9),
            Color::Indigo => (0x00, 0x1F, 0x3F),
            Color::Violet => (0xB1, 0x0D, 0xC9)
        }
    }

    fn write(&self, text: &str) -> String {
        format!("\x1b[38;2;{};{};{}m{}\x1b[0m", self.value().0, self.value().1, self.value().2, text)
    }

    fn get_next_color(&self) -> Color {
        return match *self {
            Color::Red => Color::Orange,
            Color::Orange => Color::Yellow,
            Color::Yellow => Color::Green,
            Color::Green => Color::Blue,
            Color::Blue => Color::Indigo,
            Color::Indigo => Color::Violet,
            Color::Violet => Color::Red
        }
    }
}

fn get_input_from_user() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Error while reading input!");
    input.trim().to_string()
}

fn print_rainbow_text(text: &str) {
    let mut color = Color::Red;
    for char in text.chars() {
        print!("{}", color.write(&char.to_string()));
        color = color.get_next_color();
    }
    print!("\n");
}

fn test_red_color() {
    println!("Normal {} Text", Color::Red.write("Red text"));
}