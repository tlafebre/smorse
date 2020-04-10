// https://www.reddit.com/r/dailyprogrammer/comments/cmd1hb/20190805_challenge_380_easy_smooshed_morse_code_1/

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

fn read_file(path: &PathBuf) -> String {
    fs::read_to_string(path).expect("error reading file")
}

struct MorseDictionary {
    path: PathBuf,
    pairs: HashMap<String, String>,
}

impl MorseDictionary {
    fn from(path: PathBuf) -> MorseDictionary {
        let text = read_file(&path);
        let words: Vec<_> = text.split('\n').collect();
        let pairs: HashMap<String, String> = words
            .iter()
            .map(|w| (w.to_string(), to_morse(&w)))
            .collect();

        MorseDictionary { path, pairs }
    }
}

fn to_morse(word: &str) -> String {
    let mut morse_str = String::new();
    for n in word.to_lowercase().chars() {
        let c = match n {
            'a' => ".-",
            'b' => "-...",
            'c' => "-.-.",
            'd' => "-..",
            'e' => ".",
            'f' => "..-.",
            'g' => "--.",
            'h' => "....",
            'i' => "..",
            'j' => ".---",
            'k' => "-.-",
            'l' => ".-..",
            'm' => "--",
            'n' => "-.",
            'o' => "---",
            'p' => ".--.",
            'q' => "--.-",
            'r' => ".-.",
            's' => "...",
            't' => "-",
            'u' => "..-",
            'v' => "...-",
            'w' => ".--",
            'x' => "-..-",
            'y' => "-.--",
            'z' => "--..",
            _ => "",
        };
        morse_str.push_str(c);
    }
    morse_str
}

fn group_morse_codes(codes: Vec<String>) -> HashMap<String, u8> {
    let mut counts: HashMap<String, u8> = HashMap::new();
    for item in codes.into_iter() {
        counts.entry(item).and_modify(|e| *e += 1).or_insert(1);
    }
    counts
}

fn count_occurences(s: &str, c: char) -> usize {
    s.matches(c).count()
}

fn get_code_word_pairs(s: &str) -> HashMap<String, String> {
    let words: Vec<_> = s.split('\n').collect();
    let pairs: HashMap<String, String> = words
        .into_iter()
        .map(|w| (w.to_string(), to_morse(&w.to_string())))
        .collect();
    pairs
}

fn list_morse_codes(contents: String) -> Vec<String> {
    let words: Vec<_> = contents.split('\n').collect();
    let list_of_morse_codes: Vec<String> = words
        .into_iter()
        .map(|w| (to_morse(&w.to_string())))
        .collect();

    list_of_morse_codes
}

fn code_contains_n_dashes(s: &str, n: usize) -> bool {
    let query = format!("{:-<1$}", "", n);
    s.contains(query.as_str())
}

fn word_contains_n_chars(s: &str, n: usize) -> bool {
    s.len() == n
}

fn morse_code_contains_equal_chars(code: &str) -> bool {
    let dashes: Vec<_> = code.chars().filter(|l| l == &'-').collect();
    let dots: Vec<_> = code.chars().filter(|l| l == &'.').collect();

    dashes.len() == dots.len()
}

fn morse_code_is_palindrome(code: &str) -> bool {
    let reversed: String = code.chars().rev().collect();

    reversed == code
}

fn count_total_dots_and_dashes(s: &str) {
    // For these challenges, use the enable1 word list. It contains 172,823 words.
    // If you encode them all, you would get a total of 2,499,157 dots and 1,565,081 dashes.
    let mut morse_codes_combined = String::new();
    for morse_code in &list_morse_codes(s.to_string()) {
        morse_codes_combined.push_str(&(morse_code.to_string()));
    }
    let amount_of_dots = count_occurences(&morse_codes_combined, '.');
    let amount_of_dash = morse_codes_combined.len() - amount_of_dots;

    println!(" amount of '.': {}", amount_of_dots);
    println!(" amount of '-': {}", amount_of_dash);
}

fn find_most_common_morse_code(morse_codes: Vec<String>) {
    // optional bonus challenge 1: The sequence -...-....-.--. is the code for four different words (needing, nervate, niding, tiling).
    // Find the only sequence that's the code for 13 different words.
    let mut most_common = (String::from(""), 0);
    for (code, amount) in group_morse_codes(morse_codes) {
        if amount > most_common.1 {
            most_common = (code.to_string(), amount);
        }
    }
    println!(" '{:?}' is the code for 13 words", most_common.0);
}

fn find_15_dash_string(pairs: &HashMap<String, String>) {
    // optional bonus challenge 2: autotomous encodes to .-..--------------..-..., which has 14 dashes in a row.
    // Find the only word that has 15 dashes in a row.
    for (word, code) in pairs {
        if code_contains_n_dashes(&code, 15) {
            println!(" morse code of '{}' contains 15 dashes: '{}'", word, &code)
        }
    }
}

fn find_perfectly_balanced_word(pairs: &HashMap<String, String>) {
    // optional bonus challenge 3: Call a word perfectly balanced if its code has the same number of dots as dashes.
    // counterdemonstrations is one of two 21-letter words that's perfectly balanced. Find the other one.
    for (word, code) in pairs {
        if word_contains_n_chars(&word, 21) && morse_code_contains_equal_chars(&code) {
            println!(" perfectly balanced: '{}'", &word);
        }
    }
}

fn find_palindrome_code(pairs: &HashMap<String, String>) {
    // protectorate is 12 letters long and encodes to .--..-.----.-.-.----.-..--., which is a palindrome (i.e. the string is the same when reversed).
    // Find the only 13-letter word that encodes to a palindrome.
    for (word, code) in pairs {
        if word_contains_n_chars(&word, 13) && morse_code_is_palindrome(&code) {
            println!(
                " 13 letter word: '{}' has a morse palindrome: {}",
                word, code
            );
        }
    }
}

fn main() {
    let path = PathBuf::from("data/enable1.txt");

    let morse_dict = MorseDictionary::from(path);
    let s = String::from("Tjeerd");
}
