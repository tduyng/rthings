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

    let mut input_file = File::open(input_path)
        .unwrap_or_else(|_| panic!("Unable to open file {}", input_path.display()));
    let mut input_file_text = String::new();

    input_file
        .read_to_string(&mut input_file_text)
        .expect("Unable to read file");

    let input_syntax: syn::File = syn::parse_file(&input_file_text).expect("Unable to parse file");
    let mut output_text = String::new();
    output_text.push_str(&create_initial_types());

    for item in input_syntax.items.iter() {
        match item {
            syn::Item::Type(item_type) => {
                let type_text = parse_item_type(item_type);
                output_text.push_str(&type_text);
            }
            syn::Item::Enum(item_enum) => {
                let enum_text = parse_item_enum(item_enum);
                output_text.push_str(&enum_text);
            }
            syn::Item::Struct(item_struct) => {
                let struct_text = parse_item_struct(item_struct);
                output_text.push_str(&struct_text);
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
    let mut output_text = String::new();

    output_text.push_str("export type ");
    output_text.push_str(&item_type.ident.to_string());
    output_text.push_str(" = ");

    let type_string = parse_type(&item_type.ty);
    output_text.push_str(&type_string);

    output_text
}

fn parse_type(syn_type: &syn::Type) -> String {
    let mut output_text = String::new();

    match syn_type {
        syn::Type::Path(type_path) => {
            let segment = type_path.path.segments.last().unwrap();
            let field_type = segment.ident.to_string();
            let ts_field_type = parse_type_ident(&field_type).to_owned();
            output_text.push_str(&ts_field_type);

            match &segment.arguments {
                // A simple type like i32 matches here as it
                // does not include any arguments
                syn::PathArguments::None => {}
                // Example: HashMap<String, Colour>
                syn::PathArguments::AngleBracketed(angle_bracket_args) => {
                    output_text.push_str("<");
                    let args = angle_bracket_args.args.iter();
                    for arg in args {
                        match arg {
                            syn::GenericArgument::Type(inner_type) => {
                                output_text.push_str(&parse_type(inner_type));
                                output_text.push_str(",");
                            }
                            _ => {
                                dbg!("Encountered an unimplemented token");
                            }
                        }
                    }
                    output_text.push_str(">,");
                }
                _ => {
                    dbg!("Encountered an unimplemented token");
                }
            }
        }

        syn::Type::Tuple(type_tuple) => {
            output_text.push('[');
            for elem in type_tuple.elems.iter() {
                output_text.push_str(&parse_type(elem));
                output_text.push(',');
            }
            output_text.push(']');
        }

        _ => {
            dbg!("Encountered an unimplemented token");
        }
    };

    output_text
}

fn parse_type_ident(ident: &str) -> &str {
    match ident {
        "i8" | "i16" | "i32" | "i64" | "i128" | "u8" | "u16" | "u32" | "u64" | "f32" | "f64"
        | "isize" | "usize" => "number",
        "str" | "String" | "char" => "string",
        "bool" => "boolean",
        _ => ident,
    }
}

fn parse_item_enum(item_enum: &syn::ItemEnum) -> String {
    let mut output_text = String::new();

    output_text.push_str("\n\nexport type ");
    let enum_name = item_enum.ident.to_string();
    output_text.push_str(&enum_name);
    output_text.push_str(" = ");

    for variant in item_enum.variants.iter() {
        output_text.push_str("\n\t | { t: \"");
        let variant_name = variant.ident.to_string();
        output_text.push_str(&variant_name);
        output_text.push_str("\", c: ");

        match &variant.fields {
            syn::Fields::Named(named_fields) => {
                output_text.push('{');
                for field in named_fields.named.iter() {
                    if let Some(ident) = &field.ident {
                        output_text.push_str(&ident.to_string());
                        output_text.push(':');

                        let field_type = parse_type(&field.ty);
                        output_text.push_str(&field_type);
                    }
                }
                output_text.push('}');
            }
            syn::Fields::Unnamed(unnamed_fields) => {
                // Currently only support a single unnamed field: e.g the i32 in Blue(i32)
                let unnamed_field = unnamed_fields.unnamed.first().unwrap();
                let field_type = parse_type(&unnamed_field.ty);
                output_text.push_str(&field_type);
            }
            syn::Fields::Unit => {
                output_text.push_str("undefined");
            }
        }
        output_text.push('}');
    }

    output_text
}

fn parse_item_struct(item_struct: &syn::ItemStruct) -> String {
    let mut output_text = String::new();

    let struct_name = item_struct.ident.to_string();
    output_text.push_str("\n\nexport interface ");
    output_text.push_str(&struct_name);
    output_text.push_str(" {\n");

    match &item_struct.fields {
        syn::Fields::Named(named_fields) => {
            for named_field in named_fields.named.iter() {
                match &named_field.ident {
                    Some(ident) => {
                        let field_name = ident.to_string();
                        output_text.push('\t');
                        output_text.push_str(&field_name);
                        output_text.push_str(": ");
                    }
                    None => {
                        dbg!("Encountered an unimplemented token");
                    }
                }
                let field_type = parse_type(&named_field.ty);
                output_text.push_str(&field_type);
                output_text.push('\n');
            }
        }
        // For tuple structs we will serialize them as interfaces with
        // fields named for the numerical index to align with serde's
        // default handling of this type
        syn::Fields::Unnamed(fields) => {
            // Example: struct Something (i32, Anything);
            // Output: export interface Something { 0: i32, 1: Anything }
            for (index, field) in fields.unnamed.iter().enumerate() {
                output_text.push('\t');
                output_text.push_str(&index.to_string());
                output_text.push_str(": ");
                output_text.push_str(&parse_type(&field.ty));
                output_text.push(';');
            }
        }
        syn::Fields::Unit => (),
    }
    output_text.push('}');

    output_text
}

fn create_initial_types() -> String {
    let mut output_text = String::new();

    output_text.push_str("type HashSet<T extends number | string> = Record<T, undefined>;\n");
    output_text.push_str("type HashMap<T extends number | string, U> = Record<T, U>;\n");
    output_text.push_str("type Vec<T> = Array<T>;\n");
    output_text.push_str("type Option<T> = T | undefined;\n");
    output_text.push_str("type Result<T, U> = T | U;\n");

    output_text
}



#[cfg(test)]
mod tests {

    use super::*;

    fn parse_syn_file(file: syn::File) -> String {
        let mut output_text = String::new();
    
        for item in file.items.iter() {
            match item {
                // This `Item::Type` enum variant matches our type alias
                syn::Item::Type(item_type) => {
                    let type_text = parse_item_type(item_type);
                    output_text.push_str(&type_text);
                }
                syn::Item::Enum(item_enum) => {
                    let enum_text = parse_item_enum(item_enum);
                    output_text.push_str(&enum_text);
                }
                syn::Item::Struct(item_struct) => {
                    let struct_text = parse_item_struct(item_struct);
                    output_text.push_str(&struct_text);
                }
    
                _ => {
                    dbg!("Encountered an unimplemented token");
                }
            }
        }
    
        output_text
    }

    #[test]
    fn handles_type_alias() {
        let mut input_file = File::open("./src/tests/type.rs").unwrap();

        let mut input_file_text = String::new();

        input_file.read_to_string(&mut input_file_text).unwrap();

        let input_syntax: syn::File =
            syn::parse_file(&input_file_text).expect("Unable to parse file");

        let typescript_types = parse_syn_file(input_syntax);

        assert_eq!("export type NumberAlias = number", &typescript_types);
    }

    #[test]
    fn handles_struct() {
        let mut input_file = File::open("./src/tests/struct.rs").unwrap();

        let mut input_file_text = String::new();

        input_file.read_to_string(&mut input_file_text).unwrap();

        let input_syntax: syn::File =
            syn::parse_file(&input_file_text).expect("Unable to parse file");

        let typescript_types = parse_syn_file(input_syntax);

        assert_eq!(
            "\n\nexport interface Person {\n\tname: string\n\tage: number\n\tenjoys_coffee: boolean\n}",
            &typescript_types
        );
    }

    #[test]
    fn handles_enum() {
        let mut input_file = File::open("./src/tests/enum.rs").unwrap();

        let mut input_file_text = String::new();

        input_file.read_to_string(&mut input_file_text).unwrap();

        let input_syntax: syn::File =
            syn::parse_file(&input_file_text).expect("Unable to parse file");

        let typescript_types = parse_syn_file(input_syntax);

        assert_eq!(
            "\n\nexport type Colour = \n\t | { t: \"Red\", c: number}\n\t | { t: \"Green\", c: number}\n\t | { t: \"Blue\", c: number}",
            &typescript_types
        );
    }
}
