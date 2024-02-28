use ansi_term::Color;
use hangman::hangman::Hangman;
use hangman::words;

fn main() {
    let mut n_games = 0;
    let mut won = 0;

    loop {
        n_games += 1;
        println!("\n\n\n");
        let banner = format!(
            "     ======  Game {} (won {} of {}) =====",
            n_games,
            won,
            n_games - 1
        );
        println!("{}", Color::Yellow.bold().paint(banner));
        let word = words::random_word();
        if Hangman::new(word).play() {
            won += 1;
        }
    }
}
