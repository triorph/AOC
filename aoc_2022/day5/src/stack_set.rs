use aoc_helpers::hash_utils::HashVec;
use itertools::Itertools;

pub type Instruction = (usize, usize, usize);

#[derive(Debug, Clone)]
pub struct StackSet {
    stack_set: HashVec<usize, char>,
}

impl StackSet {
    pub fn new(stack_set: HashVec<usize, char>) -> StackSet {
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
        let (quantity, source, dest) = instruction;
        for _ in 0..*quantity {
            self.move_n_from_source_to_dest(1, source, dest)
        }
    }

    fn process_move_b(&mut self, instruction: &Instruction) {
        let (quantity, source, dest) = instruction;
        self.move_n_from_source_to_dest(*quantity, source, dest)
    }

    fn move_n_from_source_to_dest(&mut self, quantity: usize, source: &usize, dest: &usize) {
        let input_stack = self.pop_n_from(source, quantity);
        self.push_to(dest, &input_stack);
    }

    fn pop_n_from(&mut self, source: &usize, quantity: usize) -> Vec<char> {
        let source_vec = self.stack_set.get_mut(source).unwrap();
        source_vec.split_off(source_vec.len() - quantity)
    }

    fn push_to(&mut self, dest: &usize, input_stack: &[char]) {
        self.stack_set.extend(*dest, input_stack)
    }

    fn get_last_value_from(&self, source: &usize) -> Option<char> {
        self.stack_set.get(source).last().copied()
    }

    pub fn show_top_values(&self) -> String {
        self.stack_set
            .keys()
            .sorted()
            .map(|name| self.get_last_value_from(name).expect("has values"))
            .collect::<String>()
    }
}
