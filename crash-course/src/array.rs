// Arrays - Fixed list where elements are the same data types

use std::mem;

pub fn run() {
  let mut numbers: [i32; 4] = [1, 2, 3, 4];
  // Re-assign value
  numbers[2] = 50;

  println!("{:?}", numbers);

  // Get single value
  println!("Single value for number2: {}", numbers[2]);

  // Get array length
  println!("Array length: {}", numbers.len());

  // Arrays are stack allocated
  println!("Stack allocated: {}", mem::size_of::<[i32; 4]>());
  println!("Array occupies {} bytes", mem::size_of_val(&numbers));

  // Get slice
  let slice: &[i32] = &numbers[1..3];
  println!("Slice: {:?}", slice);
}
//aa
// bb
