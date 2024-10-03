use std::fs::read_to_string;

use lexer::Token;

pub mod error;
pub mod lexer;
pub mod parser;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = args[1].clone();
    let files_contents = read_to_string(filename).expect("Something went wrong reading the file");
    let mut lexer = lexer::Lexer::new(&files_contents);

    loop {
        match lexer.next_token() {
            Ok(Token::EOF) => break,
            Ok(token) => println!("{:?}", token),
            Err(e) => {
                eprintln!("{}", e);
                break;
            }
        }
    }
}
