use std::{
    env,
    fs::File,
    io::{BufReader, Read},
};

use lexer::Lexer;

mod allocator;
mod lexer;
mod token;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }

    let input = &args[1];
    let file = File::open(input).expect("Could not open file");
    let mut contents = String::new();
    let mut reader = BufReader::new(file);
    reader
        .read_to_string(&mut contents)
        .expect("Could not read file");
    let mut lexer = Lexer::new(&contents);
    let tokens = lexer.lex();
    dbg!(&tokens);
}
