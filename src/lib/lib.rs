use std::fs::File;
use std::io::{self,BufReader,prelude::*};
use std::collections::HashMap;


#[derive(Debug, PartialEq)] 
pub enum Color {
    Gray,  // Absent.
    White,  // Presented, but in different position.
    Yellow, // Guessed.
}

#[derive(Debug)] 
pub struct Letter {
    color: Color,
    letter: char,
}

#[derive(Debug)] 
struct Stat {
    yellow_letters: HashMap<usize, char>,
    white_letters: HashMap<usize, char>,
    white_letters_list: Vec<char>,
    gray_letters: HashMap<usize, char>,
}

fn has_gray_letters(stats: &Stat, dict_word: &String) -> bool {
    for (i, letter) in &stats.gray_letters {
        if (*dict_word).contains(*letter) {
            return true;
        }
    }
    return false;
}

fn has_yellow_in_place(stats: &Stat, dict_word: &String) -> bool {
    for (idx, ch) in (&stats.yellow_letters).into_iter() {
        if *ch != (*dict_word).chars().nth(*idx).unwrap() {
            return false;
        }
    }
    true
}

fn is_matched(stats: &Stat, dict_word: &String) -> bool{
    if has_gray_letters(&stats, &dict_word) {
        return false;
    }
    if !has_yellow_in_place(&stats, &dict_word) {
        return false;
    }
    true
}

fn find_matches(stats: Stat) -> Vec<String> {
    let file = File::open("./data/words.txt").unwrap();
    let reader = BufReader::new(file);
    let mut matches = Vec::new();
    for word in reader.lines() {
        match word {
            Ok(w) => {
                if is_matched(&stats, &w){
                    matches.push(w);
                }
            },
            Err(_) => {},
        }
    };
    matches
}

fn to_letter(color_ch: char, ch: char) -> Letter {
    match color_ch {
        col if col == 'y' => Letter{letter: ch, color: Color::Yellow},
        col if col == 'g' => Letter{letter: ch, color: Color::White},
        col => Letter{letter: ch, color: Color::Gray},
    }
}

fn to_pattern(letter: Vec<Letter>) -> String {
    let bnd = letter.iter().filter(|l| l.color == Color::Yellow).collect::<Vec<_>>();
    let yellow_letter = bnd.first(); 
    if let Some(l) = yellow_letter {
        let s = format!("{}", l.letter);
        return s;
    } else {
        let gray_letters: Vec<&Letter> = letter.iter().filter(|l| l.color == Color::White).collect::<Vec<_>>();
        let mut s = String::from("");
        for letter in gray_letters {
            s = s + &letter.letter.to_string();
        }
        return String::from(s);
    }
}

pub fn string_to_letters(word: &String) -> Vec<Letter> {
    let chVec = word.chars().collect::<Vec<char>>();
    let mut chunks = chVec.chunks(2);
    let mut res: Vec<Letter> = Vec::new();
    for _ in 0..4 {
        match chunks.next() {
            Some(['g', ch]) => res.push(Letter{color: Color::Gray, letter: *ch}),
            Some(['w', ch]) => res.push(Letter{color: Color::White, letter: *ch}),
            Some(['y', ch]) => res.push(Letter{color: Color::Yellow, letter: *ch}),
            _ => {},
        }
    }
    res
}

pub fn strings_to_words(strings: Vec<String>) -> Vec<Vec<Letter>> {
    strings.iter().map(|string| string_to_letters(&string)).collect::<Vec<_>>()
}

pub fn guess_word(words: Vec<Vec<Letter>>) -> Vec<String> {
    let mut yellow_letters = HashMap::new();
    let mut gray_letters = HashMap::new();
    let mut white_letters = HashMap::new();
    let mut white_letters_list: Vec<char> = Vec::new();
    for word in words {
        for (idx, letter) in word.iter().enumerate() {
            match letter.color {
                Color::Yellow => {yellow_letters.entry(idx).or_insert(letter.letter);},
                Color::White => {
                    white_letters_list.push(letter.letter);
                    white_letters.entry(idx).or_insert(letter.letter);
                },
                Color::Gray => {gray_letters.entry(idx).or_insert(letter.letter);},
                // _ => panic!("Undefined color"),
            }
        }
    }

    let stats = Stat {
        yellow_letters: yellow_letters,
        gray_letters: gray_letters,
        white_letters: white_letters,
        white_letters_list: white_letters_list,
    };

    find_matches(stats)
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_has_gray_letters() {
        let stats = Stat{
            gray_letters: HashMap::from([(1, 'п'), (2, 'р')]),
            white_letters: HashMap::new(),
            white_letters_list: Vec::new(),
            yellow_letters: HashMap::new(),
        };
        assert_eq!(has_gray_letters(&stats, String::from("привет")), true);
        assert_eq!(has_gray_letters(&stats, String::from("нет")), false);
    }
}
