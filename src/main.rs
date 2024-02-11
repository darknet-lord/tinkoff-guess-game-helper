use clap::Parser;
use tinkoff_guess_game_lib::{guess_word, strings_to_words, get_suggestions};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// SHould suggest initial words
    #[arg(short, long, action)]
    suggest: bool,

    /// Used words
    #[arg(short, long, value_parser, num_args = 0.. , value_delimiter=' ')]
    words: Vec<String>,
}


fn main() {
    /*
     * The yellow letters are prefixed by `=`.
     * The gray letters have no prefixes.
     * The white letters are prefixed by `?`.
     *
     * $ cargo run --bin console_app --words =ямн=д=а
     * ябеда
     * ягода
     *
     */
    let args = Args::parse();
    if args.suggest {
        get_suggestions().iter().for_each(|s| {
            s.iter().for_each(|w| {println!("{}", w);});
            println!("-----")
        });

    } else {
        guess_word(strings_to_words(args.words)).iter().for_each(|word| {println!("{}", word);});
    }
}
