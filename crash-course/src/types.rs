/*
Primitive Types--
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays
*/

// Rust is a statically typed language,
// which means that it must know the types of all variables at compile time,
// however, the compiler can usually infer what type we want to use based on the value and how we use it.

pub fn run() {
  // Default is i32
  let x = 1;

  // Default is f64
  let y = 2.5;

  // Add explicit type
  let z: i64 = 4531513512351;

  println!("x: {}, y: {}, z: {}", x, y, z);

  // Find max size of i32
  let max_size_i32 = i32::MAX;
  let max_size_i64 = i64::MAX;
  println!("max size of i32: {}", max_size_i32);
  println!("max size of i32: {}", max_size_i64);

  // Boolean
  let is_true = true;
  println!("is true? {}", is_true);

  // Get boolean from expression
  let is_greater_than_zero = x > 0;
  println!("is greater than zero? {}", is_greater_than_zero);

  let a1 = 'a';
  let face = '\u{1F600}';
  println!("{:?}", (a1, face));
}
