use std::io;

fn main() {
    let input = get_input_from_user();
    println!("{}", get_random_word_from_string(&input));
}

fn get_input_from_user() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => String::from(input.trim()),
        Err(error) => panic!("error: {}", error)
    }
}

fn get_random_word_from_string(input: &str) -> &str {
    let mut potential_words: Vec<&str> = input.split_terminator(|char: char| !char.is_ascii_alphabetic()).collect();
    potential_words.retain(|word| !word.is_empty());

    let random_word_index: usize = rand::random::<usize>();
    let number_of_proper_words = potential_words.len();

    potential_words.get(random_word_index % number_of_proper_words).unwrap()
}