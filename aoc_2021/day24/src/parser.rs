extern crate peg;
use crate::inst::{Inst, Reg};
use crate::instructionrunner::{new_from_instructions, InstructionRunner};

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
    pub rule parse() -> InstructionRunner
        = instructions:instruction() ++ "\n"  "\n" * {
            new_from_instructions( instructions )
        }
}}
impl InstructionRunner {
    /// Generates a new InstructionRunner object to be calculated upon
    ///
    /// Inputs: the full string contents of the input data set.
    ///
    /// Returns: A new InstructionRunner object, with methods `calculate_day_a` and `calculate_day_b`
    /// available
    pub fn new(input_str: &str) -> InstructionRunner {
        day24_parser::parse(input_str).unwrap()
    }
}
