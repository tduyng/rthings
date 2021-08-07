pub fn run() {
  // Print to console
  println!("Hello from print.rs file");

  // Basic formatting
  println!("Number: {}", 1);

  // Positional arguments
  println!(
    "{0} is from {1} and {0} likes {2}",
    "Tien Duy", "Vietnam", "the programing"
  );

  // Named arguments
  println!(
    "{name} likes to play {activity}",
    name = "Tien Duy",
    activity = "Ping pong"
  );

  // Placeholders traits
  println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

  // Placeholders for debug traits
  println!("{:?}", (12, true, "hello"));

  // Basic math
  println!("10 + 10 = {}", 10 + 10);
}
