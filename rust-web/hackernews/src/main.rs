extern crate rss;

mod fetch;
use fetch::*;

fn main() {
  let result = fetch_from("https://thefullsnack.com/rss.xml");
  if result.is_ok() {
    println!("Yay! It's worked!");
  }
}
