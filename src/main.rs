use std::env;
use std::fs::File;
use std::io::BufReader;

use fhiroki_bicycle_book_wordcount::{count, CountOption};

fn main() {
    let filename = env::args().nth(1).expect("1 argument FILENAME required");
    let count_option = env::args().nth(2).expect("2 argument COUNT_OPTION required");

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(&file);

    let option = match count_option.as_str() {
        "char" => CountOption::Char,
        "word" => CountOption::Word,
        "line" => CountOption::Line,
        _ => panic!("invalid option: select from {char, word, line}")
    };

    let freqs = count(reader, option);
    println!("{:?}", freqs);
}
