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
    fn write(&self , text: &str) -> String {
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
   print_tree();
}

fn print_tree(){

    const HEIGHT: i32 = 30;
    let mut j = 1;
    let mut k = 0;
    let space = " ";
    let star = "*";

    //green

    for i in 1..HEIGHT {
        while j <= HEIGHT-i {
            j += 1;
            print!(" ")
        }
        j = 1;
        while k != 2*i-1 {
            print!("\x1b[38;5;46m*");
            k += 1;
        }
        k = 0;
        println!();
    }


    //trunk
    for i in 1..HEIGHT/5 {
        while j <= HEIGHT-i {
            j += 1;
            print!(" ");
        }
        j = 1;
        while k != 2*i-1 {
            print!("\x1b[38;5;100m*");
            k += 1;
        }
        k = 0;
        println!();
    }

}
