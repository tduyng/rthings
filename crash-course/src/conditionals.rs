// Conditionals - Used to check the condition of something and act on the result

struct Number {
  odd: bool,
  value: i32,
}

pub fn run() {
  let age: u8 = 22;
  let check_id: bool = true;
  let knows_person_of_age: bool = true;

  // If/Else
  if age > 18 && check_id || knows_person_of_age {
    println!("Bartender: What would you like to drink?");
  } else if age < 21 && check_id {
    println!("Bartender: Sorry, you have to leave");
  } else {
    println!("Bartender: I'll need to see your ID, please!");
  }

  // Shorthand If
  let is_of_age = if age >= 18 { true } else { false };
  println!("Is of age: {}", is_of_age);
}

fn fair_dice_roll() -> i32 {
  if feeling_lucky {
    6
  } else {
    4
  }
}

fn fair_dice_roll() -> i32 {
  match feeling_lucky {
    true => 6,
    false => 4,
  }
}

// let patterns can be used as conditions in if:
fn print_number(n: Number) {
  if let Number { odd: true, value } = n {
    println!("Odd number: {}", value);
  } else if let Number { odd: false, value } = n {
    println!("Even number: {}", value);
  }
}

// match arms are also patterns, just like if let:
fn print_number(n: Number) {
  match n {
    Number { odd: true, value } => println!("Odd number: {}", value),
    Number { odd: false, value } => println!("Even number: {}", value),
  }
}

// A match has to be exhaustive: at least one arm needs to match
fn print_number(n: Number) {
  match n {
    Number { value: 1, .. } => println!("One"),
    Number { value: 2, .. } => println!("Two"),
    Number { value, .. } => println!("{}", value),
    // if that last arm didn't exist, we would get a compile-time error
  }
}

// If that's hard, _ can be used as a "catch-all" pattern:
fn print_number(n: Number) {
  match n.value {
    1 => println!("One"),
    2 => println!("Two"),
    _ => println!("{}", n.value),
  }
}
