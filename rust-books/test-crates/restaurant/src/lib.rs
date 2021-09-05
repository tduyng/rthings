mod front_of_house {
  pub mod hosting {
    pub fn add_to_waitlist() {}

    pub fn seat_at_table() {}
  }

  pub mod serving {
    pub fn take_order() {}

    pub fn serve_order() {}

    pub fn take_payment() {}

    // mod back_of_house {
    //   fn fix_incorrect_order() {
    //     cook_order();
    //     super::serve_order();
    //   }

    //   fn cook_order() {}
    // }
  }
}

// use crate::front_of_house::hosting;
use self::front_of_house::hosting;

pub fn eat_at_restaurant1() {
  // Absolute path
  // crate::front_of_house::hosting::add_to_waitlist();

  // using use syntax of crate
  hosting::add_to_waitlist();

  // Relative path
  front_of_house::hosting::add_to_waitlist();
}

mod back_of_house {
  pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
  }

  impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
      Breakfast {
        toast: String::from(toast),
        seasonal_fruit: String::from("peaches"),
      }
    }
  }
}

pub fn eat_at_restaurant() {
  // Order a breakfast in the summer with Rye toast
  let mut meal = back_of_house::Breakfast::summer("Rye");

  // Change our mind about what bread we'd like
  meal.toast = String::from("Wheat");
  println!("I'd like {} toast please", meal.toast);
}

mod back_of_house2 {
  pub enum Appetizer {
    Soup,
    Salad,
  }
}

pub fn eat_at_restaurant2() {
  let order1 = back_of_house2::Appetizer::Soup;
  let order2 = back_of_house2::Appetizer::Salad;
}
