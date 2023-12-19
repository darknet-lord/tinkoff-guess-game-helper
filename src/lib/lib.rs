use std::collections::{HashMap,HashSet};
use rand::{thread_rng, Rng};
// use rand::thread_rng;
// use rand::prelude::SliceRandom;

mod words;

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
    gray_letters: HashSet<char>,
}


impl Stat {

    fn validate(&self) -> (bool, Vec<String>) {
        let mut errors = Vec::<String>::new();
        let mut wlset = HashSet::new();
        let vals = self.white_letters.values().cloned();
        for val in vals {
            for ch in val {
                if self.gray_letters.contains(&ch) {
                    errors.push(format!("White letter `{}` has been found in grays", &ch));
                };
                wlset.insert(ch);
            }
        }
        let wl_total_count = wlset.len();
        if wl_total_count > 5 {
            errors.push(format!("Too much unique white letters: {}", wl_total_count));
        }
        for ch in self.yellow_letters.values() {
            if self.gray_letters.contains(&ch) {
               errors.push(format!("Yellow letter `{}` has been found in grays", &ch));
            }
        }
        (errors.len() == 0, errors)
    }
}

fn has_gray_letters(stats: &Stat, dict_word: &str) -> bool {
    for letter in &stats.gray_letters {
        if (*dict_word).contains(*letter) {
            return true;
        }
    }
    return false;
}

fn has_yellow_in_place(stats: &Stat, dict_word: &str) -> bool {
    for (idx, ch) in (&stats.yellow_letters).into_iter() {
        if *ch != (*dict_word).chars().nth(*idx).unwrap() {
            return false;
        }
    }
    true
}

fn has_white_in_place(stats: &Stat, dict_word: &str) -> bool {
    let mut wlset = HashSet::new();
    let vals = stats.white_letters.values().cloned();
    for val in vals {
        for ch in val {
            wlset.insert(ch);
        }
    }

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
    (wlset.difference(&unique_word_chars)).count() == 0
}

fn is_matched(stats: &Stat, dict_word: &str) -> bool{
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

fn find_matches(stats: Stat) -> Vec<&'static str> {
    let mut matches = Vec::new();
    for word in words::WORDLIST.iter() {
        if is_matched(&stats, word){
            matches.push(*word);
        }
    }
    matches
}

pub fn string_to_letters(word: &String) -> Vec<Letter> {
    let chars = word.chars().collect::<Vec<char>>();
    if chars.len() > 10 || chars.len() < 5 {
        panic!("String of length from 5 to 10 chars is expected, but {} given: {}", chars.len(), word);
    }

    let mut res: Vec<Letter> = Vec::new();
    let first = chars.get(0).unwrap();
    if first.is_alphabetic() {
        res.push(Letter { color: Color::Gray, letter: *first });
    }

    let mut prev = first;
    for ch in chars.iter().skip(1) {
        if *ch != '^' || *ch != '?' || *ch != '=' {
            if ch.is_alphabetic() && (prev.is_alphabetic() || *prev == '^') {
                res.push(Letter { color: Color::Gray, letter: *ch });
            } else if *prev == '?' {
                res.push(Letter { color: Color::White, letter: *ch });
            } else if *prev == '=' {
                res.push(Letter { color: Color::Yellow, letter: *ch });
            }
        }
        prev = ch;
    }
    res
}

pub fn strings_to_words(strings: Vec<String>) -> Vec<Vec<Letter>> {
    strings.iter().map(|string| string_to_letters(&string)).collect::<Vec<_>>()
}

fn get_letters_stat(words: Vec<Vec<Letter>>) -> Stat {
    let mut yellow_letters = HashMap::new();
    let mut gray_letters = HashSet::new();
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
                Color::Gray => {gray_letters.insert(letter.letter);},
            }
        }
    }

    let stats = Stat {
        yellow_letters: yellow_letters,
        gray_letters: gray_letters,
        white_letters: white_letters,
    };

    let (success, errors) = stats.validate();
    if !success {
        panic!("Errors: {:?}", errors);
    };
    stats
}

pub fn guess_word(words: Vec<Vec<Letter>>) -> Vec<&'static str> {
    if words.len() == 0 {
        suggest_words()
    } else {
        let stats = get_letters_stat(words);
        find_matches(stats)
    }
}

pub fn suggest_words() -> Vec<&'static str> {
    let mut words = vec![
        vec!["смазь", "флейц", "будяк", "выгон", "причт"],
        vec!["плица", "гнусь", "вздор", "тюфяк", "мышей"],
        vec!["дымка", "овсец", "хлябь", "югрич", "шпунт"],
        vec!["взрыд", "аншеф", "пигус", "хлябь", "тючок"],
        vec!["шутка", "мысль", "пожня", "югрич", "въезд"],
        vec!["чужая", "кофий", "взбег", "шпынь", "хлюст"],
        vec!["фрукт", "мышца", "почин", "хлябь", "въезд"],
        vec!["удэге", "барий", "взлом", "тюфяк", "шпынь"],
        vec!["сдвиг", "щерба", "экзот", "шпынь", "муляж"],
        vec!["жучок", "знать", "мюрид", "флейц", "вспых"],
    ];
    let n = thread_rng().gen_range(0..words.len());
    words[n].clone()
}

// fn _makee_suggestions() -> Vec<&'static str> {
//     let mut words_copy = words::WORDLIST.clone();
//     words_copy.shuffle(&mut thread_rng());
//     let mut matches = Vec::new();
//     let mut tried_chars: HashSet<char> = HashSet::new();
//
//     for word in words_copy.iter() {
//         let unique_chars: HashSet<char> = HashSet::from_iter(word.chars());
//         if unique_chars.len() != 5 {
//             continue;
//         };
//         if unique_chars.difference(&tried_chars).count() < 5 {
//             continue;
//         };
//         tried_chars.extend(&unique_chars);
//         matches.push(*word);
//     }
//     matches
// }
// pub fn make_suggestions() {
//    for i in 1..1000 {
//        let suggestions = generate_suggestions();
//        if suggestions.len() > 4 {
//            println!("{:?}", suggestions);
//        }
//    }
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_has_gray_letters() {
        let stats = Stat{
            gray_letters: HashSet::from(['п', 'р']),
            white_letters: HashMap::new(),
            yellow_letters: HashMap::new(),
        };
        assert_eq!(has_gray_letters(&stats, &String::from("привет")), true);
        assert_eq!(has_gray_letters(&stats, &String::from("нет")), false);
    }

    #[test]
    fn test_has_white_in_place() {
        let stats = Stat{
            gray_letters: HashSet::new(),
            white_letters: HashMap::from([(1, vec!('п')), (0, vec!('р'))]),
            yellow_letters: HashMap::new(),
        };
        assert_eq!(has_white_in_place(&stats, &String::from("привет")), true);
    }

    #[test]
    fn test_has_white_in_place_with_wrong_stats() {
        let stats = Stat{
            gray_letters: HashSet::new(),
            white_letters: HashMap::from([(0, vec!('a', 'b', 'c')), (1, vec!('d'))]),
            yellow_letters: HashMap::new(),
        };
        assert_eq!(has_white_in_place(&stats, &String::from("dba")), false);
    }

    #[test]
    #[should_panic]
    fn test_word_to_letters_broken_format_should_panic() {
        string_to_letters(&String::from("ello"));
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
        let res = string_to_letters(&String::from("h=el=l?o"));
        assert_eq!(res[0].color, Color::Gray);
        assert_eq!(res[0].letter, 'h');
        assert_eq!(res[1].color, Color::Yellow);
        assert_eq!(res[1].letter, 'e');
    }

    #[test]
    fn test_guess_word(){
        let words = strings_to_words(vec![
            String::from("лента"),
            String::from("с=удь=я"),
            String::from("?игрок"),
        ]);
        let found_words = guess_word(words);
        let x_result = vec![String::from("мумия")];
        assert_eq!(found_words, x_result);
    }

    #[test]
    fn test_validate_white_letters() {
        let stats = Stat{
            gray_letters: HashSet::from(['c', 'h']),
            yellow_letters: HashMap::from([(0, 'h')]),
            white_letters: HashMap::from([(0, vec!('a', 'b', 'c', 'd', 'e', 'f'))]),
        };
        let (success, errors) = stats.validate();
        let x_errors = vec![
            String::from("White letter `c` has been found in grays"),
            String::from("Too much unique white letters: 6"),
            String::from("Yellow letter `h` has been found in grays"),
        ];
        assert_eq!(success, false);
        assert_eq!(errors, x_errors);
    }
}
