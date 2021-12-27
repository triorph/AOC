extern crate peg;
use itertools::Itertools;

pub struct Day24Setup {
    instructions: Vec<Inst>,
}

enum Reg {
    W,
    X,
    Y,
    Z,
    Value(isize),
}

enum Inst {
    Inp(Reg),
    Add(Reg, Reg),
    Mul(Reg, Reg),
    Div(Reg, Reg),
    Mod(Reg, Reg),
    Eql(Reg, Reg),
}

struct ALU {
    inputs: Vec<usize>,
    inputs_asked: usize,
    w: isize,
    x: isize,
    y: isize,
    z: isize,
}

peg::parser! { grammar day24_parser() for str {
    rule positive_number() -> isize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule negative_number() -> isize
        = "-" n:positive_number() { -n }
    rule reg_value() -> Reg
        = n:(positive_number() / negative_number()) { Reg::Value(n) }
    rule reg_w() -> Reg
        = "w" { Reg::W }
    rule reg_x() -> Reg
        = "x" { Reg::X }
    rule reg_y() -> Reg
        = "y" { Reg::Y }
    rule reg_z() -> Reg
        = "z" { Reg::Z }
    rule extra_register() -> Reg
        = register:(reg_w() / reg_x() / reg_y() / reg_z() / reg_value()) { register }
    rule main_register() -> Reg
        = register:(reg_w() / reg_x() / reg_y() / reg_z()) { register }
    rule input() -> Inst
        = "inp " register:main_register() { Inst::Inp (register) }
    rule add() -> Inst
        = "add " reg1:main_register() " " reg2:extra_register() { Inst::Add(reg1, reg2) }
    rule multiply() -> Inst
        = "mul " reg1:main_register() " " reg2:extra_register() { Inst::Mul(reg1, reg2) }
    rule divide() -> Inst
        = "div " reg1:main_register() " " reg2:extra_register() { Inst::Div(reg1, reg2) }
    rule modulo() -> Inst
        = "mod " reg1:main_register() " " reg2:extra_register() { Inst::Mod(reg1, reg2) }
    rule equal() -> Inst
        = "eql " reg1:main_register() " " reg2:extra_register() { Inst::Eql(reg1, reg2) }
    rule instruction() -> Inst
        = instruction:(input() / add() / multiply() / divide() / modulo() / equal()) { instruction }
    pub rule parse() -> Day24Setup
        = instructions:instruction() ++ "\n"  "\n" * {
            Day24Setup {instructions}
        }
}}

impl ALU {
    fn new(inputs: Vec<usize>) -> ALU {
        ALU {
            inputs,
            inputs_asked: 0,
            w: 0,
            x: 0,
            y: 0,
            z: 0,
        }
    }

    fn get_for_register(&self, register: &Reg) -> isize {
        match register {
            &Reg::W => self.w,
            &Reg::X => self.x,
            &Reg::Y => self.y,
            &Reg::Z => self.z,
            &Reg::Value(val) => val,
        }
    }

    fn set_for_register(&mut self, register: &Reg, value: isize) {
        match register {
            &Reg::W => self.w = value,
            &Reg::X => self.x = value,
            &Reg::Y => self.y = value,
            &Reg::Z => self.z = value,
            &Reg::Value(_) => panic!("Cannot set to a value register"),
        }
    }

    fn input(&mut self, register: &Reg) -> bool {
        self.set_for_register(register, self.inputs[self.inputs_asked] as isize);
        self.inputs_asked += 1;
        if self.z == 0 {
            false
        } else {
            true
        }
    }

    fn add(&mut self, reg1: &Reg, reg2: &Reg) -> bool {
        let val = self.get_for_register(reg1) + self.get_for_register(reg2);
        self.set_for_register(reg1, val);
        true
    }

    fn multiply(&mut self, reg1: &Reg, reg2: &Reg) -> bool {
        let val = self.get_for_register(reg1) + self.get_for_register(reg2);
        self.set_for_register(reg1, val);
        true
    }

    fn divide(&mut self, reg1: &Reg, reg2: &Reg) -> bool {
        let val = self.get_for_register(reg1) / self.get_for_register(reg2);
        self.set_for_register(reg1, val);
        true
    }

    fn modulo(&mut self, reg1: &Reg, reg2: &Reg) -> bool {
        let val = self.get_for_register(reg1) % self.get_for_register(reg2);
        self.set_for_register(reg1, val);
        true
    }

    fn equal(&mut self, reg1: &Reg, reg2: &Reg) -> bool {
        let val = match self.get_for_register(reg1) == self.get_for_register(reg2) {
            true => 1,
            false => 0,
        };
        self.set_for_register(reg1, val);
        true
    }

    fn run_instruction(&mut self, instruction: &Inst) -> bool {
        match instruction {
            Inst::Inp(reg) => self.input(&reg),
            Inst::Add(reg1, reg2) => self.add(&reg1, &reg2),
            Inst::Mul(reg1, reg2) => self.multiply(&reg1, &reg2),
            Inst::Div(reg1, reg2) => self.divide(&reg1, &reg2),
            Inst::Mod(reg1, reg2) => self.modulo(&reg1, &reg2),
            Inst::Eql(reg1, reg2) => self.equal(&reg1, &reg2),
        }
    }

    fn run_instructions(&mut self, instructions: &Vec<Inst>) -> bool {
        for instruction in instructions {
            self.run_instruction(instruction);
        }
        self.z == 0
    }
}

impl Day24Setup {
    /// Generates a new Day24Setup object to be calculated upon
    ///
    /// Inputs: the full string contents of the input data set.
    ///
    /// Returns: A new Day24Setup object, with methods `calculate_day_a` and `calculate_day_b`
    /// available
    pub fn new(input_str: &str) -> Day24Setup {
        day24_parser::parse(input_str).unwrap()
    }

    fn build_actual_number(&self, num_array: &[usize]) -> usize {
        let mut ret = 0;
        for num in num_array.iter() {
            ret *= 10;
            ret += num
        }
        ret
    }

    fn increment_actual_number(&self, mut num_array: Vec<usize>) -> Vec<usize> {
        for i in 0..14 {
            if num_array[13 - i] == 1 {
                num_array[13 - i] = 9;
            } else {
                num_array[13 - i] -= 1;
                break;
            }
        }
        num_array
    }

    /// Calculate the part a response
    pub fn calculate_day_a(self: &Day24Setup) -> usize {
        let mut found = false;
        let mut count = 0;
        let mut ret: Vec<usize> = vec![9; 14];
        while !found {
            count += 1;
            if count % 1000 == 0 {
                println!("Trying number {}", self.build_actual_number(&ret[..]));
            }
            let mut alu = ALU::new(ret.clone());
            if alu.run_instructions(&self.instructions) {
                found = true;
            } else {
                ret = self.increment_actual_number(ret);
            }
        }
        self.build_actual_number(&ret[..])
    }

    /// Calculate the part b response
    pub fn calculate_day_b(self: &Day24Setup) -> usize {
        0
    }
}

#[cfg(test)]
mod test {
    use crate::Day24Setup;

    #[test]
    fn test_parse() {
        let _day24_setup = Day24Setup::new(include_str!("../test_data.txt"));
    }

    #[test]
    fn test_day_a() {
        let day24_setup = Day24Setup::new(include_str!("../test_data.txt"));
        assert_eq!(day24_setup.calculate_day_a(), 0);
    }

    #[test]
    fn test_day_b() {
        let day24_setup = Day24Setup::new(include_str!("../test_data.txt"));
        assert_eq!(day24_setup.calculate_day_b(), 0);
    }
}
