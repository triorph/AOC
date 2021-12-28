extern crate peg;

pub struct Day24Setup {
    instructions: Vec<Inst>,
}

#[derive(Debug)]
enum Reg {
    W,
    X,
    Y,
    Z,
    Value(isize),
}

#[derive(Debug)]
enum Inst {
    Inp(Reg),
    Add(Reg, Reg),
    Mul(Reg, Reg),
    Div(Reg, Reg),
    Mod(Reg, Reg),
    Eql(Reg, Reg),
}

#[derive(Clone, Debug)]
enum MonadInst {
    Value(isize),
    Input(usize),
    Add(Box<MonadInst>, Box<MonadInst>),
    Mul(Box<MonadInst>, Box<MonadInst>),
    Div(Box<MonadInst>, Box<MonadInst>),
    Mod(Box<MonadInst>, Box<MonadInst>),
    Eql(Box<MonadInst>, Box<MonadInst>),
}

#[derive(Debug)]
struct Alu {
    inputs_asked: usize,
    possible_values_for_inputs: Vec<Vec<usize>>,
    w: MonadInst,
    x: MonadInst,
    y: MonadInst,
    z: MonadInst,
}

impl MonadInst {
    fn is_zero(&self) -> bool {
        matches!(self, MonadInst::Value(0))
    }

    fn is_one(&self) -> bool {
        matches!(self, MonadInst::Value(1))
    }

    fn is_literal(&self) -> bool {
        matches!(self, MonadInst::Value(_))
    }

    fn get_literal_value(&self) -> isize {
        if let MonadInst::Value(val) = self {
            *val
        } else {
            panic!("Must call get_literal_value on a MonadInst::Value")
        }
    }

    fn literal_equal(&self, other: &MonadInst) -> MonadInst {
        match (self, other) {
            (MonadInst::Value(s), MonadInst::Value(o)) => {
                if s == o {
                    MonadInst::Value(1)
                } else {
                    MonadInst::Value(0)
                }
            }
            _ => MonadInst::Value(0),
        }
    }

    fn is_literal_invalid_for_input(&self) -> bool {
        if let MonadInst::Value(val) = self {
            !(1..=9).contains(val)
        } else {
            false
        }
    }

    fn is_input(&self) -> bool {
        matches!(self, MonadInst::Input(_))
    }
}

impl std::ops::Add for &MonadInst {
    type Output = MonadInst;
    fn add(self, other: &MonadInst) -> MonadInst {
        if self.is_zero() {
            other.clone()
        } else if other.is_zero() {
            self.clone()
        } else if self.is_literal() && other.is_literal() {
            MonadInst::Value(self.get_literal_value() + other.get_literal_value())
        } else {
            MonadInst::Add(Box::new(self.clone()), Box::new(other.clone()))
        }
    }
}

impl std::ops::Mul for &MonadInst {
    type Output = MonadInst;
    fn mul(self, other: &MonadInst) -> MonadInst {
        if self.is_zero() || other.is_zero() {
            MonadInst::Value(0)
        } else if self.is_one() {
            other.clone()
        } else if other.is_one() {
            self.clone()
        } else if self.is_literal() && other.is_literal() {
            MonadInst::Value(self.get_literal_value() * other.get_literal_value())
        } else {
            MonadInst::Mul(Box::new(self.clone()), Box::new(other.clone()))
        }
    }
}

impl std::ops::Div for &MonadInst {
    type Output = MonadInst;
    fn div(self, other: &MonadInst) -> MonadInst {
        if self.is_zero() {
            MonadInst::Value(0)
        } else if other.is_one() {
            self.clone()
        } else if self.is_literal() && other.is_literal() {
            MonadInst::Value(self.get_literal_value() / other.get_literal_value())
        } else {
            MonadInst::Div(Box::new(self.clone()), Box::new(other.clone()))
        }
    }
}

impl std::ops::Rem for &MonadInst {
    type Output = MonadInst;
    fn rem(self, other: &MonadInst) -> MonadInst {
        if self.is_zero() {
            MonadInst::Value(0)
        } else if self.is_literal() && other.is_literal() {
            MonadInst::Value(self.get_literal_value() % other.get_literal_value())
        } else {
            MonadInst::Mod(Box::new(self.clone()), Box::new(other.clone()))
        }
    }
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

impl Alu {
    fn new() -> Alu {
        let possible_values_for_inputs = vec![(1..=9).collect::<Vec<usize>>()];
        Alu {
            inputs_asked: 0,
            possible_values_for_inputs,
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
        println!("Asking for inputs, and we look like: {:?}", self);
        self.inputs_asked += 1;
        !self.z.is_zero()
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

    fn equal(&mut self, reg1: &Reg, reg2: &Reg) -> bool {
        // To do: reduce these to 1 or 0, but doing a binary split on possible values for the
        // various types when doing so, and exploring all 1s/0s where a possible type still exists.
        let left = self.get_for_register(reg1);
        let right = self.get_for_register(reg2);
        if left.is_literal() && right.is_literal() {
            self.set_for_register(reg1, left.literal_equal(&right));
        } else if left.is_input() && right.is_literal_invalid_for_input()
            || right.is_input() && left.is_literal_invalid_for_input()
        {
            self.set_for_register(reg1, MonadInst::Value(0));
        } else {
            self.set_for_register(reg1, MonadInst::Eql(Box::new(left), Box::new(right)));
        }
        true
    }

    fn run_instruction(&mut self, instruction: &Inst) -> bool {
        match instruction {
            Inst::Inp(reg) => self.input(reg),
            Inst::Add(reg1, reg2) => self.add(reg1, reg2),
            Inst::Mul(reg1, reg2) => self.multiply(reg1, reg2),
            Inst::Div(reg1, reg2) => self.divide(reg1, reg2),
            Inst::Mod(reg1, reg2) => self.modulo(reg1, reg2),
            Inst::Eql(reg1, reg2) => self.equal(reg1, reg2),
        }
    }

    fn run_instructions(&mut self, instructions: &[Inst]) -> bool {
        for instruction in instructions {
            println!("Adding instruction {:?} to ALU", instruction);
            self.run_instruction(instruction);
            println!("ALU looks like: {:?}", self.inputs_asked);
        }
        self.z.is_zero();
        true
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
            let mut alu = Alu::new();
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
