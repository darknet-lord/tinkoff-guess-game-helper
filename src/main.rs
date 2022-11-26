use std::env;
use tinkoff_guess_game_lib::{guess_word, strings_to_words};


fn main() {
    /*
     * The yellow letters are prefixed by `y`.
     * The gray letters are prefixed by `g`.
     * The white letters are prefixed by `w`.
     *
     * $ cargo run --bin console_app yяgмgнyдyа
     * ябеда
     * ягода
     *
     */
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() == 0 {
        panic!("Usage: cargo run yяgмgнyдyа")
    }
    let words = strings_to_words(args);
    for word in guess_word(words) {
        println!("{}", word);
    };
}
