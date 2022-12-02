use aoc_helpers::{AOCCalculator, AOCFileOrParseError};
use day1::Day1;
use day2::Day2;

fn main() -> Result<(), AOCFileOrParseError> {
    // TODO: allow choosing which day to filter
    Day1::new("day1/data/input_data.txt")?.print_results("day1");
    Day2::new("day2/data/input_data.txt")?.print_results("day2");
    Ok(())
}
