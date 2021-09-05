// #[derive(Debug)]
// struct User {
//   username: String,
//   email: String,
//   sign_in_count: u64,
//   active: bool,
// }
#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }

  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }
}

fn main() {
  let rect1 = Rectangle {
    width: 30,
    height: 50,
  };

  println!(
    "The area of the rectangle is {} square pixels.",
    rect1.area()
  );
  let rect2 = Rectangle {
    width: 10,
    height: 40,
  };
  let rect3 = Rectangle {
    width: 60,
    height: 45,
  };

  println!("Can rect1 hold rect2 ? {}", rect1.can_hold(&rect2));
  println!("Can rect1 hold rect3 ? {}", rect1.can_hold(&rect3));
}

// fn main() {
//   let rect = Rectangle {
//     width: 30,
//     height: 50,
//   };

//   println!(
//     "Area of rectangle with width: {0} and height: {1} is {2}",
//     rect.width,
//     rect.height,
//     get_area(&rect)
//   );
//   println!("Rectangle is {:#?}", rect);
// }

// fn get_area(rect: &Rectangle) -> u32 {
//   rect.width * rect.height
// }

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
  Alabama,
  Alaska,
}

enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
  match coin {
    Coin::Penny => 1,
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter(state) => {
      println!("State quarter from {:?}!", state);
      25
    }
  }
}

// the _ placeholder
// Rust also has a pattern we can use when we don’t want to list all possible values.
// For example, a u8 can have valid values of 0 through 255.
// If we only care about the values 1, 3, 5, and 7,
// we don’t want to have to list out 0, 2, 4, 6, 8, 9 all the way up to 255.
// Fortunately, we don’t have to: we can use the special pattern _ instead:

// fn main() {
//   let some_u8_value = 0u8;
//   match some_u8_value {
//     1 => println!("one"),
//     3 => println!("three"),
//     5 => println!("five"),
//     7 => println!("seven"),
//     _ => (),
//   }
// }
