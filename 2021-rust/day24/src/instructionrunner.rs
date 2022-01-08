use crate::alu::Alu;
use crate::inst::Inst;

pub struct InstructionRunner {
    instructions: Vec<Inst>,
}

pub fn new_from_instructions(instructions: Vec<Inst>) -> InstructionRunner {
    InstructionRunner { instructions }
}

impl InstructionRunner {
    /// Calculate the part a response
    ///
    /// Part a: Calculate the maximum 14-digit number that gives z = 0 at the end of the ALU processing
    /// step, where each digit must be 1-9
    pub fn calculate_day_a(self: &InstructionRunner) -> usize {
        let mut alu = Alu::new();
        alu.run_instructions(&self.instructions, 0);
        alu.possible_values_for_inputs
            .iter()
            .map(|input_possibilities| input_possibilities.iter().max().unwrap())
            .fold(0, |acc, val| {
                let acc = acc * 10;
                acc + val
            })
    }

    /// Calculate the part b response
    ///
    /// Part b: Calculate the minimum 14-digit number that gives z = 0 at the end of the ALU processing
    /// step, where each digit must be 1-9
    pub fn calculate_day_b(self: &InstructionRunner) -> usize {
        let mut alu = Alu::new();
        alu.run_instructions(&self.instructions, 0);
        alu.possible_values_for_inputs
            .iter()
            .map(|input_possibilities| input_possibilities.iter().min().unwrap())
            .fold(0, |acc, val| {
                let acc = acc * 10;
                acc + val
            })
    }
}

#[cfg(test)]
mod test {
    use crate::InstructionRunner;

    #[test]
    fn test_parse() {
        let _day24_setup = InstructionRunner::new(include_str!("../test_data.txt"));
    }

    #[test]
    fn test_day_a() {
        // Borrowed from a reddit solution + input
        let day24_setup = InstructionRunner::new(include_str!("../test_data.txt"));
        assert_eq!(day24_setup.calculate_day_a(), 41299994879959);
    }

    #[test]
    fn test_day_b() {
        // Borrowed from a reddit solution + input
        let day24_setup = InstructionRunner::new(include_str!("../test_data.txt"));
        assert_eq!(day24_setup.calculate_day_b(), 11189561113216);
    }
}
