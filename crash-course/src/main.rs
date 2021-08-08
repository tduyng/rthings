mod print;
mod string;
mod tuple;
mod types;
mod vars;
mod array;

fn main() {
  print::run();
  vars::run();
  types::run();
  string::run();
  tuple::run();
  array::run();
}
