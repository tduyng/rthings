// Structs - Used to create custom data types

// Traditional struct
// struct Point {
//   x: i32,
//   y: i32,
// }

struct Color {
  red: u8,
  green: u8,
  blue: u8,
}

struct Vec2 {
  x: f64, // 64-bit floating point, aka "double precision"
  y: f64,
}

// Tuple struct
struct ColorT(u8, u8, u8);

struct Person {
  first_name: String,
  last_name: String,
}

impl Person {
  // Constructor person
  fn new(first: &str, last: &str) -> Person {
    Person {
      first_name: first.to_string(),
      last_name: last.to_string(),
    }
  }

  // Get full name
  fn full_name(&self) -> String {
    format!("{} {}", self.first_name, self.last_name)
  }

  // Set last name
  fn set_last_name(&mut self, last: &str) {
    self.last_name = last.to_string();
  }

  // Name to tuple
  fn to_tuple(self) -> (String, String) {
    (self.first_name, self.last_name)
  }
}

pub fn run() {
  let mut c = Color {
    red: 255,
    green: 0,
    blue: 0,
  };
  c.red = 200;
  println!("Color: {} {} {}", c.red, c.green, c.blue);

  let mut ct = ColorT(255, 0, 0);
  ct.0 = 200;
  println!("Color: {} {} {}", ct.0, ct.1, ct.2);

  let mut p = Person::new("Mary", "Doe");
  println!("Person {}", p.full_name());
  p.set_last_name("Williams");
  println!("Person {}", p.full_name());
  println!("Person Tuple {:?}", p.to_tuple());

  let v1 = Vec2 { x: 1.0, y: 3.0 };
  let v2 = Vec2 { y: 2.0, x: 4.0 };
  // the order does not matter, only the names do
  let v3 = Vec2 { x: 14.0, ..v2 };
  let v4 = Vec2 { ..v3 };

  println!("{:?}", v1);
  // println!("v2: {:?}", v2);
  // println!("v3: {:?}", v3);

  let v = Vec2 { x: 3.0, y: 6.0 };
  let Vec2 { x, y } = v;
  // `x` is now 3.0, `y` is now `6.0`
}
