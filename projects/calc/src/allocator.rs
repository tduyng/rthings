use crate::values::Value;

const MAX_REGISTER: usize = 128;

#[derive(Debug)]
pub struct Allocator {
    registers: [bool; MAX_REGISTER],
}

impl Allocator {
    pub fn new() -> Self {
        Self {
            registers: [false; MAX_REGISTER],
        }
    }

    pub fn allocate(&mut self) -> Option<usize> {
        for (index, is_allocated) in self.registers.iter_mut().enumerate() {
            if !*is_allocated {
                *is_allocated = true;
                return Some(index + 1);
            }
        }
        None
    }

    pub fn dealloc(&mut self, index: usize) {
        if self.registers[index - 1] {
            self.registers[index - 1] = false;
        } else {
            panic!(
                "r{} was not allocated, why is it being deallocated?",
                index + 1
            )
        }
    }
}

#[derive(Debug)]
pub struct Pool {
    pub constants: Vec<Value>,
}

impl Pool {
    pub fn alloc(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }
}
