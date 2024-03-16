use core::panic;
use std::collections::HashMap;
use std::env::args;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader, Error, Write};

fn main() -> Result<(), Error> {
    let args: Vec<String> = args().collect();
    if !(args.len() > 1) {
        panic!("No file path passed");
    }

    let words_file_path = &args[1];
    let words_file = File::open(words_file_path)?;
    let words_buffer = BufReader::new(words_file);

    let mut words: Vec<String> = vec![];

    for word in words_buffer.lines() {
        words.push(word?);
    }

    let mut letter_keys = HashMap::new();
    letter_keys.insert(2, vec!['a', 'b', 'c']);
    letter_keys.insert(3, vec!['d', 'e', 'f']);
    letter_keys.insert(4, vec!['g', 'h', 'i']);
    letter_keys.insert(5, vec!['j', 'k', 'l']);
    letter_keys.insert(6, vec!['m', 'n', 'o']);
    letter_keys.insert(7, vec!['p', 'q', 'r', 's']);
    letter_keys.insert(8, vec!['t', 'u', 'v']);
    letter_keys.insert(9, vec!['w', 'x', 'y', 'z']);

    println!("Betű:");
    let mut letter = String::new();
    stdin().read_line(&mut letter)?;

    fn get_letter_key(letter: char, letter_keys: &HashMap<u8, Vec<char>>) -> String {
        for (key, value) in letter_keys {
            if value.contains(&letter) {
                let index = value.iter().position(|&r| r == letter).unwrap();
                return key.to_string().repeat(index + 1);
            }
        }
        return "".to_string();
    }

    println!(
        "{}",
        get_letter_key(letter.chars().next().unwrap(), &letter_keys)
    );

    println!("");
    println!("Szó:");

    let mut word = String::new();
    stdin().read_line(&mut word)?;

    let mut word_string = String::new();
    for letter in word.to_lowercase().chars() {
        word_string = format!("{}{}", word_string, get_letter_key(letter, &letter_keys));
    }

    println!("{}", word_string);

    let mut longest_word: String = words[0].clone();
    for word in &words {
        if word.len() > longest_word.len() {
            longest_word = word.to_string();
        }
    }

    println!("");
    println!(
        "A leghosszabb szó a(z) {}, hosszúsága {} karakter.",
        longest_word,
        longest_word.len()
    );

    println!("");
    println!(
        "Az állományban {} rövid szó található.",
        words
            .clone()
            .into_iter()
            .filter(|x| x.len() < 6)
            .collect::<Vec<String>>()
            .len()
    );

    let mut word_keys: Vec<String> = vec![];

    for word in &words {
        let mut word_string = String::new();

        for letter in word.chars() {
            word_string = format!("{}{}", word_string, get_letter_key(letter, &letter_keys));
        }

        word_keys.push(word_string);
    }

    let word_keys_file_path = "kodok.txt";
    let mut word_keys_file = File::create(word_keys_file_path)?;
    for word_key in &word_keys {
        write!(word_keys_file, "{}\n", word_key)?;
    }

    Ok(())
}
