use std::cmp::max;
use std::cmp::min;

// // this also works:
// use std::cmp::{min, max};

// // this also works!
// use std::{cmp::min, cmp::max};

// // this brings `min` and `max` in scope, and many other things
// use std::cmp::*;

pub fn run() {
  // let least = std::cmp::min(3, 8); // this is 3
  let least = min(7, 1); // this is 1
}
