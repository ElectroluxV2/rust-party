fn main() {
    println!("{}", rgb_wrap("Goodbye cruel world", 255, 0, 0));
}

fn rgb_wrap(text: &str, r: u8, g: u8, b: u8) -> String {
    format!("\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, text)
}
