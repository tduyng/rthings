use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

use lexer::Lexer;

mod lexer;
mod token;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }

    let input = &args[1];
    let file_path = env::current_dir()
        .expect("Could not get current directory")
        .join(input);

    let file = match File::open(&file_path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error opening file {}: {}", file_path.display(), err);
            std::process::exit(1);
        }
    };

    let reader = BufReader::new(file);
    let mut lexer = Lexer::new(reader.lines());
    lexer.lex();
}
