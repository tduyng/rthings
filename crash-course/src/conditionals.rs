// Conditionals - Used to check the condition of something and act on the result

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
