// trait is other name of interface in other programming languages
// trait will be use for implementing struct

// Our trait on a foreign type (a primitive type, even):

trait Signed {
  fn is_strictly_negative(self) -> bool;
  fn to_tuple(self) -> (bool, i32);
}
trait Signed2 {
  fn is_strictly_negative(self) -> bool;
}

struct Number {
  odd: bool,
  value: i32,
}

impl Signed for Number {
  fn is_strictly_negative(self) -> bool {
    self.value < 0
  }
  fn to_tuple(self) -> (bool, i32) {
    (self.odd, self.value)
  }
}

impl Signed2 for i32 {
  fn is_strictly_negative(self) -> bool {
    self < 0
  }
}

// A foreign trait on our type: negative
// the `Neg` trait is used to overload `-`, the
// unary minus operator.
impl std::ops::Neg for Number {
  type Output = Number;

  fn neg(self) -> Number {
    Number {
      value: -self.value,
      odd: self.odd,
    }
  }
}

// An impl block is always for a type, so, inside that block, Self means that type:
// impl std::ops::Neg for Number {
//   type Output = Self;

//   fn neg(self) -> Self {
//       Self {
//           value: -self.value,
//           odd: self.odd,
//       }
//   }
// }

pub fn run() {
  let n = Number {
    odd: false,
    value: -44,
  };
  println!("{:?}", (&n.odd, &n.value));
  println!("{}", &n.is_strictly_negative()); // prints "true"
                                             // println!("{:?}", &n.to_tuple()); // prints "-44")

  let x: i32 = -44;
  println!("{}", x.is_strictly_negative()); // prints "true"

  let y = Number {
    odd: true,
    value: 987,
  };
  let z = -y; // this is only possible because we implemented `Neg`
  println!("{}", z.value); // prints "-987"
}
