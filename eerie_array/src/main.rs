use std::io;

enum Operations {
    A,
    E,
    P,
    M,
    Q
}

impl Operations {
    fn value(&self) -> (char) {
        match*self {
            Operations::A => A,
            Operations::E => E,
            Operations::P => P,
            Operations::M => M,
            Operations::Q => Q
        }
    }

}

fn main() {
    let mut length = 0;
    let mut array = vec![0; length];
}

fn get_input_from_user() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error not readable input");
    input.trim().to_string()
}
