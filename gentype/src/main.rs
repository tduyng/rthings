use clap::{Arg, Command};
use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

fn main() {
    let matches = Command::new("Gentype")
        .version("0.1.0")
        .author("Felix N")
        .about("Covert Rust types to Typescript types")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .required(true)
                .help("The Rust file to process (including ext"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .required(true)
                .help("The name of Typescript file to output (including ext"),
        )
        .get_matches();

    let input_filename = matches.get_one::<String>("input").expect("input required");
    let output_filename = matches
        .get_one::<String>("output")
        .expect("output required");

    dbg!(input_filename);
    dbg!(output_filename);

    let input_path = Path::new(input_filename);
    dbg!(input_path.display());
    let mut input_file =
        File::open(input_path).unwrap_or_else(|_| panic!("Unable to open file {}", input_path.display()));
    let mut input_file_text = String::new();

    input_file
        .read_to_string(&mut input_file_text)
        .expect("Unable to read file");

    let input_syntax: syn::File = syn::parse_file(&input_file_text).expect("Unable to parse file");
    let mut output_text = String::new();

    for item in input_syntax.items.iter(){
        match item {
            syn::Item::Type(item_type) =>{
                let type_text = parse_item_type(item_type);
                output_text.push_str(&type_text);
            }
            _ => {
                dbg!("Encountered an unimplemented type");
            }
        }
    }

    let mut output_file = File::create(output_filename).unwrap();

    write!(output_file, "{}", output_text).expect("Failed to write to output file");
}

fn parse_item_type(item_type: &syn::ItemType) -> String {
    String::from("todo")
}
