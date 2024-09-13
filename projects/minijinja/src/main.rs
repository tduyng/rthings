mod filters;
mod macros;
mod parser;
mod values;
mod vm;

fn main() {
    let template = "Hello, {{ name }}!";

    let mut parser = parser::Parser::new(template);
    let ast = parser.parse().unwrap();

    let mut context = vm::Context::new();
    context.set(
        "name".to_string(),
        values::Value::String("Alice".to_string()),
    );

    let vm = vm::Vm::new();
    let result = vm.execute(&ast, &context);

    println!("{}", result);
}
