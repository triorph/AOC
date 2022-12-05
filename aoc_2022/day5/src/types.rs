use std::collections::HashMap;
pub type Move = (usize, usize, usize);
pub type MoveList = Vec<Move>;
pub type Stack = Vec<char>;
pub type StackSet = HashMap<usize, Stack>;

pub trait CanMoveStacks {
    fn process_moves_a(&mut self, moves: &[Move]);
    fn process_move_a(&mut self, stack_move: &Move);
    fn process_moves_b(&mut self, moves: &[Move]);
    fn process_move_b(&mut self, stack_move: &Move);
    fn show_top_values(&self) -> String;
}

impl CanMoveStacks for StackSet {
    fn process_moves_a(&mut self, moves: &[Move]) {
        for stack_move in moves.iter() {
            self.process_move_a(stack_move);
        }
    }

    fn process_moves_b(&mut self, moves: &[Move]) {
        for stack_move in moves.iter() {
            self.process_move_b(stack_move);
        }
    }

    fn process_move_a(&mut self, stack_move: &Move) {
        let (quantity, source, destination) = stack_move;
        for _ in 0..*quantity {
            let val = {
                let source_v: &mut Vec<char> = self.get_mut(source).unwrap();
                (*source_v).pop()
            };
            if let Some(val) = val {
                let dest_v: &mut Vec<char> = self.get_mut(destination).unwrap();
                dest_v.push(val);
            }
        }
    }

    fn process_move_b(&mut self, stack_move: &Move) {
        let (quantity, source, destination) = stack_move;
        let mut input_stack = {
            let mut input_stack = Vec::new();
            let source_v: &mut Vec<char> = self.get_mut(source).unwrap();
            for _ in 0..*quantity {
                input_stack.push((*source_v).pop());
            }
            input_stack
        };
        let dest_v: &mut Vec<char> = self.get_mut(destination).unwrap();
        for _ in 0..*quantity {
            if let Some(val) = input_stack.pop().unwrap() {
                dest_v.push(val);
            }
        }
    }

    fn show_top_values(&self) -> String {
        let mut ret = String::new();
        let mut names: Vec<usize> = self.keys().copied().collect();
        names.sort();
        for name in names.iter() {
            ret.push(
                *self
                    .get(name)
                    .expect("keys are in dict")
                    .last()
                    .expect("has values"),
            )
        }
        ret
    }
}
