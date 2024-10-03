use std::fs::read_to_string;

pub mod error;
pub mod json_value;
pub mod lexer;
pub mod parser;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = args[1].clone();
    let files_contents = read_to_string(filename).expect("Something went wrong reading the file");

    let lexer = lexer::Lexer::new(&files_contents);
    let mut parser = parser::Parser::new(lexer);

    match parser.parse() {
        Ok(json_value) => println!("{}", json_value),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
