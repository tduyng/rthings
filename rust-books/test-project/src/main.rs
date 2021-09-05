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

fn main() {
  let rect = Rectangle {
    width: 30,
    height: 50,
  };

  println!(
    "Area of rectangle with width: {0} and height: {1} is {2}",
    rect.width,
    rect.height,
    get_area(&rect)
  );
  println!("Rectangle is {:#?}", rect);
}

fn get_area(rect: &Rectangle) -> u32 {
  rect.width * rect.height
}
