mod print;
mod string;
mod tuple;
mod types;
mod vars;

fn main() {
  print::run();
  vars::run();
  types::run();
  string::run();
  tuple::run();
}
