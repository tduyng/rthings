pub fn run() {
  let name: &str = "Tien Duy";
  let mut age = 20;
  let height = 1.7;

  println!(
    "My name is {}, I am {} years old and I'm {} tall.",
    name, age, height
  );

  age = 25;
  println!(
    "My name is {}, I am {} years old and I'm {} tall.",
    name, age, height
  );

  // Define const
  const PI: f64 = 3.1415;
  const ID: i32 = 42;
  println!("The value of PI is {} and ID: {}", PI, ID);

  // Assign multiple variables
  let (m, n) = (1, 2);
  println!("m: {} and n: {}", m, n);
}
