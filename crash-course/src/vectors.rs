// Vectors - Resizable arrays

use std::mem;

// This works because Rust inserts this at the beginning of every module:

// Rust code
// use std::prelude::v1::*;
// (Which in turns re-exports a lot of symbols, like Vec, String, Option and Result).

pub fn run() {
  let mut numbers: Vec<i32> = vec![1, 2, 3];

  // Re-assign value
  numbers[2] = 20;

  // Add on to vector
  numbers.push(5);
  numbers.push(6);

  // Pop off last value
  numbers.pop();

  println!("{:?}", numbers);

  // Get single val
  println!("Single Value: {}", numbers[0]);

  // Get vector length
  println!("Vector Length: {}", numbers.len());

  // Vectors are heap allocated
  println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

  // Get Slice
  let slice: &[i32] = &numbers[1..3];
  println!("Slice: {:?}", slice);

  // Loop through vector values
  for number in numbers.iter() {
    println!("Number: {}", number);
  }

  // Loop & mutate values
  for x in numbers.iter_mut() {
    *x *= 2;
  }

  println!("Numbers Vec: {:?}", numbers);

  // `Vec` is a regular struct, not a primitive type
  let mut v = Vec::new();
  println!("Vec: {:?}", v);

  // // this is exactly the same code, but with the *full* path to `Vec`
  // let v = std::vec::Vec::new();
  v.push(1);
  v.push(10);
  v.push(20);

  println!("Vec: {:?}", v);
}
