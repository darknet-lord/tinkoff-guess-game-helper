use std::fs::File;
use std::io::{BufReader,prelude::*};
use std::collections::{HashMap,HashSet};

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
    white_letters: HashMap<usize, Vec<char>>,
    gray_letters: HashMap<usize, char>,
}

fn has_gray_letters(stats: &Stat, dict_word: &String) -> bool {
    for (_, letter) in &stats.gray_letters {
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

fn has_white_in_place(stats: &Stat, dict_word: &String) -> bool {
    let mut wlset = HashSet::new();
    let vals = stats.white_letters.values().cloned();
    for val in vals {
        for ch in val {
            wlset.insert(ch);
        }
    }

    let mut wl_total_count = wlset.len();
    for (idx, ch) in dict_word.chars().enumerate() {
        if stats.yellow_letters.contains_key(&idx) {
            continue;
        } else {
            if stats.white_letters.contains_key(&idx) && stats.white_letters[&idx].contains(&ch) {
                return false;
            }
        }
    }
    let unique_word_chars = HashSet::from_iter(dict_word.chars());
    (wlset.difference(&unique_word_chars)).count() >= 0
}

fn is_matched(stats: &Stat, dict_word: &String) -> bool{
    if has_gray_letters(&stats, &dict_word) {
        return false;
    }
    if !has_yellow_in_place(&stats, &dict_word) {
        return false;
    }
    if !has_white_in_place(&stats, &dict_word) {
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

pub fn string_to_letters(word: &String) -> Vec<Letter> {
    let chars = word.chars().collect::<Vec<char>>();
    if chars.len() != 10 {
        panic!("String of length 10 is expected, but {} given: {}", chars.len(), word);
    }
    let mut chunks = chars.chunks(2);
    let mut res: Vec<Letter> = Vec::new();
    for _ in 0..5 {
        match chunks.next() {
            Some(['g', ch]) => res.push(Letter{color: Color::Gray, letter: *ch}),
            Some(['w', ch]) => res.push(Letter{color: Color::White, letter: *ch}),
            Some(['y', ch]) => res.push(Letter{color: Color::Yellow, letter: *ch}),
            _ => {panic!("Invalid format: {}", word);},
        }
    }
    res
}

pub fn strings_to_words(strings: Vec<String>) -> Vec<Vec<Letter>> {
    strings.iter().map(|string| string_to_letters(&string)).collect::<Vec<_>>()
}

fn get_letters_stat(words: Vec<Vec<Letter>>) -> Stat {
    let mut yellow_letters = HashMap::new();
    let mut gray_letters = HashMap::new();
    let mut white_letters: HashMap<usize, Vec<char>> = HashMap::new();
    for word in words {
        for (idx, letter) in word.iter().enumerate() {
            match letter.color {
                Color::Yellow => {yellow_letters.entry(idx).or_insert(letter.letter);},
                Color::White => {
                    if white_letters.contains_key(&idx) {
                        white_letters.get_mut(&idx).map(|val| val.push(letter.letter));
                    } else {
                        white_letters.insert(idx, vec![letter.letter]);
                    };
                },
                Color::Gray => {gray_letters.entry(idx).or_insert(letter.letter);},
                // _ => panic!("Undefined color"),
            }
        }
    }

    Stat {
        yellow_letters: yellow_letters,
        gray_letters: gray_letters,
        white_letters: white_letters,
    }
}

pub fn guess_word(words: Vec<Vec<Letter>>) -> Vec<String> {
    let stats = get_letters_stat(words);
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
            yellow_letters: HashMap::new(),
        };
        assert_eq!(has_gray_letters(&stats, &String::from("привет")), true);
        assert_eq!(has_gray_letters(&stats, &String::from("нет")), false);
    }

    #[test]
    fn test_has_white_in_place() {
        let stats = Stat{
            gray_letters: HashMap::new(),
            white_letters: HashMap::from([(1, vec!('п')), (0, vec!('р'))]),
            yellow_letters: HashMap::new(),
        };
        assert_eq!(has_white_in_place(&stats, &String::from("привет")), true);
    }

    #[test]
    #[should_panic]
    fn test_word_to_letters_broken_format_should_panic() {
        string_to_letters(&String::from("hello"));
    }

    #[test]
    #[should_panic]
    fn test_word_to_letters_empty_should_panic() {
        string_to_letters(&String::from(""));
    }

    #[test]
    #[should_panic]
    fn test_word_to_letters_too_long_should_panic() {
        string_to_letters(&String::from("ghyeglylwowo"));
    }

    #[test]
    fn test_word_to_letters() {
        let res = string_to_letters(&String::from("ghyeglylwo"));
        assert_eq!(res[0].color, Color::Gray);
        assert_eq!(res[0].letter, 'h');
        assert_eq!(res[1].color, Color::Yellow);
        assert_eq!(res[1].letter, 'e');
    }

    #[test]
    fn test_guess_word(){
        let words = strings_to_words(vec![
            String::from("gлgеgнgтgа"),
            String::from("gсyуgдgьyя"),
            String::from("wиgгgрgоgк"),
        ]);
        let found_words = guess_word(words);
        let x_result = vec![
            String::from("гурия"),
            String::from("курия"),
            String::from("курья"),
            String::from("мумия"),
            String::from("мурья"),
            String::from("рупия"),
            String::from("судья"),
            String::from("фурия"),
        ];
        assert_eq!(found_words, x_result);
    }
}

