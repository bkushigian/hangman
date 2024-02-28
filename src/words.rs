use rand;

pub fn random_word() -> String {
    // Get a random word from resources/dictionary.txt
    let words = include_str!("../resources/dictionary-tom25-hangman.txt");
    let words: Vec<&str> = words.lines().collect();
    let word = words[rand::random::<usize>() % words.len()];
    return word.to_ascii_uppercase().to_string();
}
