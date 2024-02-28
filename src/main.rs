use hangman::hangman::Hangman;
use hangman::words;

fn main() {
    let word = words::random_word();
    Hangman::new(word).play();
}
