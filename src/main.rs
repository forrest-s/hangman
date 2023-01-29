extern crate rand;
use std::io;
mod word;

fn main() {
    let word_length = word::get_word_length();
    let word_to_guess = word::get_random_word(word_length);
    let mut letters_guessed = vec![];
    let mut incorrect_guesses = 0;
    let mut word_display = vec!['_'; word_to_guess.len()];

    while incorrect_guesses < 6 {
        println!("Word to guess: {:?}", word_display);
        println!("Incorrect guesses: {}", incorrect_guesses);
        println!("Letters guessed: {:?}", letters_guessed);

        println!("Please enter your guess:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read guess");

        guess = guess.trim().to_string();

        if guess.len() != 1 {
            println!("Please enter a single letter!");
            continue;
        }

        if letters_guessed.contains(&guess) {
            println!("You've already guessed that letter, try again!");
            continue;
        }

        letters_guessed.push(guess.clone());

        if word_to_guess.contains(guess.as_str()) {
            for (i, c) in word_to_guess.chars().enumerate() {
                if c == guess.chars().next().unwrap() {
                    word_display[i] = c;
                }
            }

            if !word_display.contains(&'_') {
                println!("Congratulations! You guessed the word: {}", word_to_guess);
                break;
            }
        } else {
            incorrect_guesses += 1;

            if incorrect_guesses == 6 {
                println!("Sorry, you didn't guess the word in time. The word was: {}", word_to_guess);
                break;
            }
        }
    }
}
