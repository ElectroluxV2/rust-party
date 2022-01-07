fn main() {
    print_rgb("Goodbye Cruel World", (0xFF, 0x00, 0x00));
}

fn print_rgb(text: &str, color: (u8, u8, u8)) {
    println!("\x1b[38;2;{};{};{}m{}\x1b[0m", color.0, color.1, color.2, text);
}