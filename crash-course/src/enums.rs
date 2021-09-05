// ## Enums are types which have a few definite values

enum Movement {
  // Variants
  Up,
  Down,
  Left,
  Right,
}

fn move_avatar(m: Movement) {
  // Perform action depending on info
  match m {
    Movement::Up => println!("Avatar moving up"),
    Movement::Down => println!("Avatar moving down"),
    Movement::Left => println!("Avatar moving left"),
    Movement::Right => println!("Avatar moving right"),
  }
}

pub fn run() {
  let avatar1 = Movement::Left;
  let avatar2 = Movement::Up;
  let avatar3 = Movement::Right;
  let avatar4 = Movement::Down;

  move_avatar(avatar1);
  move_avatar(avatar2);
  move_avatar(avatar3);
  move_avatar(avatar4);
}

// ## Concise control flow with if let

fn main() {
  let some_u8_value = Some(0u8);
  match some_u8_value {
    Some(3) => println!("three"),
    _ => (),
  }
}

// We can do with other way: more concise with if let
// The syntax if let takes a pattern and an expression separated by an equal sign. 
// It works the same way as a match, where the expression is given to the match and the pattern is its first arm.

let some_u8_value = Some(0u8);
if let Some(3) = some_u8_value {
    println!("three");
}

