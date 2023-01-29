use rand::seq::SliceRandom;
use std::io;

pub fn get_word_length() -> u8 {
    println!("Enter the number of letters in the word you would like to guess (3-10): ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let input = input.trim().parse::<u8>().unwrap();

    if input < 3 || input > 10 {
        println!("Invalid input. Please enter a number between 3 and 10.");
        get_word_length()
    } else {
        input
    }
}

pub fn get_random_word(length: u8) -> String {
    let three_letters = ["cat", "dog", "rat", "man", "sun"];
    let four_letters = ["moon", "fire", "bird", "tree", "rock"];
    let five_letters = ["water", "table", "light", "plant", "peace"];
    let six_letters = ["planet", "orange", "person", "laptop", "family"];
    let seven_letters = ["airport", "picture", "musical", "natural", "weather"];
    let eight_letters = ["computer", "building", "adventure", "mountain", "creature"];
    let nine_letters = ["vegetable", "beautiful", "invention", "creativity", "difficulty"];
    let ten_letters = ["technology", "astronomer", "impossible", "understand", "leadership"];

    let word_list = match length {
        3 => three_letters,
        4 => four_letters,
        5 => five_letters,
        6 => six_letters,
        7 => seven_letters,
        8 => eight_letters,
        9 => nine_letters,
        10 => ten_letters,
        _ => return "Invalid number entered, please try again".to_string(),
    };

    let mut rng = rand::thread_rng();
    let random_word = word_list.choose(&mut rng).unwrap();
    return random_word.to_string();
}
