use std::collections::HashMap;

pub type Instruction = (usize, usize, usize);
pub type Stack = Vec<char>;

#[derive(Debug, Clone)]
pub struct StackSet {
    stack_set: HashMap<usize, Stack>,
}

impl StackSet {
    pub fn new(stack_set: HashMap<usize, Stack>) -> StackSet {
        StackSet { stack_set }
    }
    pub fn process_moves_a(&mut self, instructions: &[Instruction]) {
        for instruction in instructions.iter() {
            self.process_move_a(instruction);
        }
    }

    pub fn process_moves_b(&mut self, instructions: &[Instruction]) {
        for instruction in instructions.iter() {
            self.process_move_b(instruction);
        }
    }

    fn process_move_a(&mut self, instruction: &Instruction) {
        let (quantity, source, destination) = instruction;
        for _ in 0..*quantity {
            let val = self.pop_from(source);
            if let Some(val) = val {
                self.push_to(destination, val);
            }
        }
    }

    fn process_move_b(&mut self, instruction: &Instruction) {
        let (quantity, source, destination) = instruction;
        let mut input_stack = Vec::new();
        for _ in 0..*quantity {
            if let Some(val) = self.pop_from(source) {
                input_stack.push(val);
            }
        }
        for val in input_stack.iter().rev() {
            self.push_to(destination, *val);
        }
    }

    fn pop_from(&mut self, source: &usize) -> Option<char> {
        self.stack_set.get_mut(source).unwrap().pop()
    }

    fn push_to(&mut self, dest: &usize, value: char) {
        self.stack_set.get_mut(dest).unwrap().push(value);
    }

    pub fn show_top_values(&self) -> String {
        let mut ret = String::new();
        let mut names: Vec<usize> = self.stack_set.keys().copied().collect();
        names.sort();
        for name in names.iter() {
            ret.push(
                *self
                    .stack_set
                    .get(name)
                    .expect("keys are in dict")
                    .last()
                    .expect("has values"),
            )
        }
        ret
    }
}
