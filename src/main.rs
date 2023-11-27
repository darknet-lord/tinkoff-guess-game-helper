use std::env;
use tinkoff_guess_game_lib::{guess_word, strings_to_words};


fn main() {
    /*
     * The yellow letters are prefixed by `=`.
     * The gray letters have no prefixes.
     * The white letters are prefixed by `?`.
     *
     * $ cargo run --bin console_app =ямн=д=а
     * ябеда
     * ягода
     *
     */

    let args: Vec<String> = env::args().skip(1).collect();
    let words = strings_to_words(args);
    for word in guess_word(words) {
        println!("{}", word);
    };
}
