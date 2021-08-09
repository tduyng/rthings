// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run() {
  let mut hello = String::from("Hello ");

  // Push char
  hello.push('w');

  // Push string
  hello.push_str("orld!");

  // Get length
  println!("Length: {}", hello.len());

  // Capacity in bytes
  println!("Capacity: {}", hello.capacity());

  // Check if empty
  println!("hello is empty?: {}", hello.is_empty());

  // Contains
  println!("hello contains Hello?: {}", hello.contains("Hello"));

  // Replace
  println!(
    "Replace world by there: {}",
    hello.replace("world", "there")
  );

  // Loop through string by white space
  for c in hello.split_whitespace() {
    println!("{}", c);
  }

  // Create string with capacity
  let mut hello2 = String::with_capacity(10);
  hello2.push_str("Hello2  ");
  hello2.push_str("world!");

  // Assertion testing
  println!("{}", hello2.len());
  println!("{}", hello2.capacity());
  assert_eq!(14, hello2.len());
  assert_eq!(20, hello2.capacity());

  println!("{}, {}", hello, hello2);

  // Types are namespaces too, and methods can be called as regular functions:
  let x = "amos".len(); // this is 4
  let x = str::len("amos"); // this is also 4
}
