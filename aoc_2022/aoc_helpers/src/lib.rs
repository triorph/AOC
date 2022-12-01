use std::fs::read_to_string;

pub trait AOCCalculator<Answer> {
    fn new(input_file: &str) -> Self;
    fn calculate_day_a(&self) -> Answer;
    fn calculate_day_b(&self) -> Answer;
    fn print_results(&self, name: &str);
}

pub fn read_input_file(filename: &str) -> String {
    read_to_string(filename)
        .unwrap_or_else(|_| panic!("We required {:?} to exist for this to run", filename))
}
