mod array;
mod conditionals;
mod print;
mod string;
mod tuple;
mod types;
mod vars;
mod vectors;

fn main() {
  print::run();
  vars::run();
  types::run();
  string::run();
  tuple::run();
  array::run();
  vectors::run();
  conditionals::run();
}
