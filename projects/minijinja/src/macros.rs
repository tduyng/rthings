use crate::{ast::Ast, values::Value};
use std::collections::HashMap;

#[derive(Clone)]
pub struct Macro {
    pub name: String,
    pub params: Vec<String>,
    pub body: Ast,
}

pub struct MacroRegistry {
    macros: HashMap<String, Macro>,
}

impl MacroRegistry {
    pub fn new() -> MacroRegistry {
        MacroRegistry {
            macros: HashMap::new(),
        }
    }

    pub fn define_macro(&mut self, name: &str, params: Vec<String>, body: Ast) {
        let new_macro = Macro {
            name: name.to_string(),
            params,
            body,
        };
        self.macros.insert(name.to_string(), new_macro);
    }

    pub fn call_macro(
        &self,
        name: &str,
        args: Vec<Value>,
        vm: &mut crate::vm::Vm,
    ) -> Result<String, String> {
        if let Some(macro_def) = self.macros.get(name) {
            if macro_def.params.len() != args.len() {
                return Err(format!(
                    "Expected {} arguments, but got {}",
                    macro_def.params.len(),
                    args.len()
                ));
            }

            // Temporarily borrow vm mutably
            let result = {
                let vm = vm; // mutable borrow is used here

                // Set macro parameters in the VM context
                for (param, arg) in macro_def.params.iter().zip(args.into_iter()) {
                    vm.set_variable(param, arg);
                }

                // Render the macro body
                vm.render(&macro_def.body)
            };

            result
        } else {
            Err(format!("Macro {} not found", name))
        }
    }
}

impl Clone for MacroRegistry {
    fn clone(&self) -> MacroRegistry {
        MacroRegistry {
            macros: self.macros.clone(),
        }
    }
}
