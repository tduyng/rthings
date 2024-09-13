mod ast;
mod filters;
mod macros;
mod parser;
mod values;
mod vm;

use filters::FilterRegistry;
use macros::MacroRegistry;
use parser::Parser;
use values::Value;
use vm::Vm;

fn main() {
    // Complex template with various features:
    let template = r#"
        Hello, {{ name | upper }}!
        
        {% if is_logged_in %}
            Welcome back, {{ name }}!
        {% else %}
            Please log in.
        {% endif %}
        
        {% macro greet_user(greeting, user) %}
            {{ greeting }}, {{ user }}!
        {% endmacro %}
        
        {{ greet_user('Hi', name) }}
        
        {% for item in items %}
            Item: {{ item }}
        {% endfor %}
    "#;

    // Set up the parser to tokenize and parse the template
    let mut parser = Parser::new(template);
    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("Error parsing template: {}", e);
            return;
        }
    };

    // Prepare the runtime environment
    let mut vm = Vm::new();

    // Set up context variables to simulate data passed to the template
    vm.set_variable("name", Value::String("Alice".to_string()));
    vm.set_variable("is_logged_in", Value::Bool(true));
    vm.set_variable(
        "items",
        Value::List(vec![
            Value::String("Item 1".to_string()),
            Value::String("Item 2".to_string()),
            Value::String("Item 3".to_string()),
        ]),
    );

    // Register macros
    let macro_registry = MacroRegistry::new();
    vm.set_macro_registry(macro_registry);

    // Register filters
    let filter_registry = FilterRegistry::new();
    vm.set_filter_registry(filter_registry);

    // Render the template
    match vm.render(&ast) {
        Ok(output) => println!("Rendered output:\n{}", output),
        Err(e) => eprintln!("Error rendering template: {}", e),
    }
}
