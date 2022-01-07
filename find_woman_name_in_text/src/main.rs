use std::io;

fn main() {
    const TARGET: &str = "Laura";

    loop {
        let input = get_input_from_user();
        let found_woman = get_woman_name_from_text(&input);
        if found_woman.eq(&Some(TARGET.to_string())) {
            println!("Found target name, exiting");
            break;
        }
        print_message_based_on_name_presence(found_woman);
    }
}

fn get_input_from_user() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => String::from(input.trim()),
        Err(error) => panic!("error: {}", error)
    }
}

fn get_woman_name_from_text(text: &str) -> Option<String> {
    let words = text.split_whitespace();

    for word in words {
        if word.starts_with(|c: char| c.is_uppercase()) && word.ends_with('a') {
            return Some(String::from(word));
        }
    }

    None
}

fn print_message_based_on_name_presence(name: Option<String>) {
    match name {
        Some(woman) => println!("Found woman name \"{}\"", woman),
        None => println!("No woman name in text")
    }
}