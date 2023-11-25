use std::io;

fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5f64 / 3f64; // Results in -1

    // remainder
    let remainder = 43 % 5;

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    let arr = [1, 2, 3, 4, 5];

    println!(
        "
    | sum: {sum} 
    | difference: {difference} 
    | product: {product} 
    | quotient: {quotient} 
    | truncated: {truncated} 
    | remainder: {remainder}
    | c: {c}
    | z: {z}
    | heart_eyed_cat: {heart_eyed_cat}
    "
    );
    println!("arr: {:?}", arr);

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = arr[index];

    println!("The value of the element at index {index} is: {element}");
}
