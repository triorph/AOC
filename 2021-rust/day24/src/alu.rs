use itertools::Itertools;
use std::collections::HashSet;

use crate::inst::{Inst, Reg};
use crate::monadinst::MonadInst;

#[derive(Debug, Clone)]
pub struct Alu {
    inputs_asked: usize,
    pub possible_values_for_inputs: Vec<Vec<usize>>,
    done: bool,
    w: MonadInst,
    x: MonadInst,
    y: MonadInst,
    z: MonadInst,
}

impl<'a> Alu {
    pub fn new() -> Alu {
        let possible_values_for_inputs = vec![(1..=9).collect::<Vec<usize>>(); 14];
        Alu {
            inputs_asked: 0,
            possible_values_for_inputs,
            done: false,
            w: MonadInst::Value(0),
            x: MonadInst::Value(0),
            y: MonadInst::Value(0),
            z: MonadInst::Value(0),
        }
    }

    fn get_for_register(&self, register: &Reg) -> MonadInst {
        match register {
            Reg::W => self.w.clone(),
            Reg::X => self.x.clone(),
            Reg::Y => self.y.clone(),
            Reg::Z => self.z.clone(),
            Reg::Value(val) => MonadInst::Value(*val),
        }
    }

    fn set_for_register(&mut self, register: &Reg, value: MonadInst) {
        match register {
            Reg::W => self.w = value,
            Reg::X => self.x = value,
            Reg::Y => self.y = value,
            Reg::Z => self.z = value,
            Reg::Value(_) => panic!("Cannot set to a value register"),
        }
    }

    fn input(&mut self, register: &Reg) -> bool {
        self.set_for_register(register, MonadInst::Input(self.inputs_asked));
        self.inputs_asked += 1;
        true
    }

    fn add(&mut self, reg1: &Reg, reg2: &Reg) -> bool {
        let val = &self.get_for_register(reg1) + &self.get_for_register(reg2);
        self.set_for_register(reg1, val);
        true
    }

    fn multiply(&mut self, reg1: &Reg, reg2: &Reg) -> bool {
        let val = &self.get_for_register(reg1) * &self.get_for_register(reg2);
        self.set_for_register(reg1, val);
        true
    }

    fn divide(&mut self, reg1: &Reg, reg2: &Reg) -> bool {
        let val = &self.get_for_register(reg1) / &self.get_for_register(reg2);
        self.set_for_register(reg1, val);
        true
    }

    fn modulo(&mut self, reg1: &Reg, reg2: &Reg) -> bool {
        let val = &self.get_for_register(reg1) % &self.get_for_register(reg2);
        self.set_for_register(reg1, val);
        true
    }

    fn reduce_possibilities(&mut self, reg1: &Reg, reg2: &Reg, expected_value: bool) -> bool {
        // Filter the possible input values based on an equals expression and an expected value.
        // At the point of doing this, we should be checking Input(X) + C = Input(Y). The program
        // will panic if this is not so.
        //
        // Returns false if any input gets reduced to having no possible values.
        let left = self.get_for_register(reg1);
        let right = self.get_for_register(reg2);
        let left_input = left.get_input_operating_on();
        let right_input = right.get_input_operating_on();
        let left_possibilities = &self.possible_values_for_inputs[left.get_input_operating_on()];
        let right_possibilities = &self.possible_values_for_inputs[right.get_input_operating_on()];
        let possible_values: Vec<(usize, usize)> = left_possibilities
            .iter()
            .cartesian_product(right_possibilities)
            .filter(|(l, r)| (left.evaluate(**l) == right.evaluate(**r)) == expected_value)
            .map(|(l, r)| (*l, *r))
            .collect();
        let left_found = possible_values
            .iter()
            .map(|v| v.0)
            .collect::<HashSet<usize>>();
        let right_found = possible_values
            .iter()
            .map(|v| v.1)
            .collect::<HashSet<usize>>();
        self.possible_values_for_inputs[left_input] = (&self.possible_values_for_inputs
            [left_input])
            .iter()
            .copied()
            .filter(|val| left_found.contains(val))
            .collect::<Vec<usize>>();
        self.possible_values_for_inputs[right_input] = (&self.possible_values_for_inputs
            [right_input])
            .iter()
            .copied()
            .filter(|val| right_found.contains(val))
            .collect::<Vec<usize>>();
        self.possible_values_for_inputs
            .iter()
            .all(|p| !p.is_empty())
    }

    fn equal(&mut self, reg1: &Reg, reg2: &Reg, instructions: &[Inst], i: usize) -> bool {
        let left = self.get_for_register(reg1);
        let right = self.get_for_register(reg2);

        if left.is_literal() && right.is_literal() {
            self.set_for_register(reg1, left.literal_equal(&right));
        } else if left.is_input() && right.is_literal_invalid_for_input()
            || right.is_input() && left.is_literal_invalid_for_input()
        {
            // Quickly eliminate any checks of Input(X) == Value for values not in 1..9
            self.set_for_register(reg1, MonadInst::Value(0));
        } else {
            // At this point, all the equals are: input(X) + b == input(Y)

            // Create a clone, and see if we can make this equality be true.
            let mut clone_with_equal_1 = self.clone();
            if clone_with_equal_1.reduce_possibilities(reg1, reg2, true) {
                clone_with_equal_1.set_for_register(reg1, MonadInst::Value(1));
                if clone_with_equal_1.run_instructions(instructions, i + 1) {
                    self.possible_values_for_inputs =
                        clone_with_equal_1.possible_values_for_inputs.clone();
                    self.z = MonadInst::Value(0);
                    self.done = true;
                    return true;
                }
            }
            // If we are at this point, then our clone failed to pan out. Proceed with equality
            // result of false
            let ret = self.reduce_possibilities(reg1, reg2, false);
            self.set_for_register(reg1, MonadInst::Value(0));
            return ret;
        }
        true
    }

    fn run_instruction(&mut self, instructions: &[Inst], i: usize) -> bool {
        match &instructions[i] {
            Inst::Inp(reg) => self.input(reg),
            Inst::Add(reg1, reg2) => self.add(reg1, reg2),
            Inst::Mul(reg1, reg2) => self.multiply(reg1, reg2),
            Inst::Div(reg1, reg2) => self.divide(reg1, reg2),
            Inst::Mod(reg1, reg2) => self.modulo(reg1, reg2),
            Inst::Eql(reg1, reg2) => self.equal(reg1, reg2, instructions, i),
        }
    }

    pub fn run_instructions(&mut self, instructions: &[Inst], start: usize) -> bool {
        for i in start..(instructions.len()) {
            if self.done {
                break;
            }
            if !self.run_instruction(instructions, i) {
                return false;
            }
        }
        if self.z.is_zero() {
            self.done = true;
            true
        } else {
            false
        }
    }
}
