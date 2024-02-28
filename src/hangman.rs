use ansi_term::Color;
use std::io::{self, Write};

pub struct Hangman {
    word: String,
    correct_guesses: Vec<char>,
    incorrect_guesses: Vec<char>,
}

impl Hangman {
    pub fn new(word: String) -> Hangman {
        Hangman {
            word,
            correct_guesses: Vec::new(),
            incorrect_guesses: Vec::new(),
        }
    }

    pub fn is_done(&self) -> bool {
        self.is_dead() || self.is_complete()
    }

    pub fn is_dead(&self) -> bool {
        self.incorrect_guesses.len() >= 8
    }

    pub fn is_complete(&self) -> bool {
        for letter in self.word.chars() {
            if !self.correct_guesses.contains(&letter) {
                return false;
            }
        }
        true
    }

    pub fn guess(&mut self, letter: char) {
        if self.correct_guesses.contains(&letter) || self.incorrect_guesses.contains(&letter) {
            return;
        }
        if self.word.contains(letter) {
            self.correct_guesses.push(letter);
        } else {
            self.incorrect_guesses.push(letter);
        }
    }

    pub fn display(&self) -> String {
        let mut display = String::new();
        for letter in self.word.chars() {
            if self.correct_guesses.contains(&letter) {
                display.push(letter);
            } else {
                display.push('-');
            }
        }
        display
    }

    pub fn draw_all_letters_one_line(&self) {
        let letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        for letter in letters.chars() {
            if self.incorrect_guesses.contains(&letter) {
                print!("{} ", Color::Red.paint(letter.to_string()));
            } else if self.correct_guesses.contains(&letter) {
                print!("{} ", Color::Green.paint(letter.to_string()));
            } else {
                print!("{} ", letter.to_string());
            }
        }
        println!("");
    }

    pub fn draw_all_letters_multi_line(&self) {
        let letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        println!("+---------------------------  GUESSES  --------------------------+");
        print!("| {}: ", Color::Red.paint("incorrect"));
        for letter in letters.chars() {
            if self.incorrect_guesses.contains(&letter) {
                print!("{} ", Color::Red.paint(letter.to_string()));
            } else {
                print!("  ");
            }
        }
        println!("|");
        print!("| {}: ", Color::Green.paint("  correct"));
        for letter in letters.chars() {
            if self.correct_guesses.contains(&letter) {
                print!("{} ", Color::Green.paint(letter.to_string()));
            } else {
                print!("  ");
            }
        }
        println!("|");
        print!("| unguessed: ");
        for letter in letters.chars() {
            if !self.correct_guesses.contains(&letter) && !self.incorrect_guesses.contains(&letter)
            {
                print!("{} ", letter.to_string());
            } else {
                print!("  ");
            }
        }
        println!("|");
        println!("+----------------------------------------------------------------+");
    }

    pub fn draw(&self) {
        let live_features = vec!["(_)", "\\", "|", "/", "|", "/", "\\"];
        let dead_features = vec!["(_)", "/", "|", "\\", "|", "|", "|"];
        let mut to_draw = vec!["", "", "", "", "", "", ""];
        let num_guesses = self.incorrect_guesses.len();
        if num_guesses > live_features.len() {
            for i in 0..dead_features.len() {
                to_draw[i] = dead_features[i];
            }
        } else {
            for i in 0..num_guesses {
                to_draw[i] = live_features[i];
            }
        }

        let yellow = Color::Yellow;

        println!(" ____ ");
        println!("|/   |");
        println!("|   {}", to_draw[0]);
        println!("|   {}{}{}", to_draw[1], to_draw[2], to_draw[3]);
        println!("|    {}", to_draw[4]);
        println!("|   {} {}", to_draw[5], to_draw[6]);
        println!("|      ");
        println!("|_____ ");

        println!("");
        println!("{}", yellow.bold().paint(self.display()));
    }

    pub fn play(&mut self) {
        while !self.is_done() {
            println!("\n\n\n\n\n\n\n\n\n\n\n\n");
            self.draw();
            self.draw_all_letters_multi_line();
            print!("{}: ", Color::Yellow.bold().paint("Guess a letter"));
            io::stdout().flush().unwrap();

            let mut guess = String::new();
            std::io::stdin().read_line(&mut guess).unwrap();
            let guess = guess.trim().to_ascii_uppercase().chars().next().unwrap();
            self.guess(guess);
        }
        if self.is_complete() {
            println!("You win!");
            println!(
                "The word was: {}",
                Color::Green.bold().paint(self.word.to_ascii_uppercase())
            );
        } else {
            self.draw();
            println!(
                "You lose! The word was: {}",
                Color::Red
                    .bold()
                    .blink()
                    .paint(self.word.to_ascii_uppercase())
            );
        }
    }
}
