use std::fmt;
use Terminal::{Color, Clear, Position};

enum Terminal {
    Color(u8, u8, u8),
    Clear(),
    Position(i8, i8)
}

impl Terminal {
    fn format(&self, text: &str) -> String {
        match self{
            Color(r, g, b) => {
                format!(
                "\x1b[38;2;{};{};{}m{}\x1b[0m",
                r,
                g,
                b,
                text)
            }
            Clear() => { format!("\x1b[H\x1b[2J\x1b[3J") }
            Position(_, _) => {"".to_string()}
        }
    }
}

struct Palette {
    red: Terminal,
    blue: Terminal,
}

const PALETTE: Palette = Palette {
    red: Color(255, 65, 54),
    blue: Color(0, 191, 255),
};


fn main() {
    print_tree();
}

fn print_tree() {
    const HEIGHT: i32 = 30;
    let mut j = 1;
    let mut k = 0;
    let star = "*";
    let ball = "O";

    print!("{}",Clear().format(""));

    //green

    for i in 1..HEIGHT {
        while j <= HEIGHT - i {
            j += 1;
            print!(" ");
        }
        j = 1;
        while k != 2 * i - 1 {
            print!("{}", PALETTE.blue.format(star));
            k += 1;
        }
        k = 0;
        println!();
    }

    //trunk
    for i in 3..HEIGHT / 4 {
        while j <= HEIGHT - i {
            j += 1;
            print!(" ");
        }
        j = 1;
        while k != 2 * i - 1 {
            print!("{}", PALETTE.blue.format(star));
            k += 1;
        }
        k = 0;
        println!();
    }
}
