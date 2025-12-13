use aoc_helpers::AOCCalculator;
use color_eyre::eyre::Result;
use day1::Day01;
use day10::Day10;
use day11::Day11;
use day12::Day12;
use day2::Day2;
use day3::Day3;
use day4::Day4;
use day5::Day5;
use day6::Day6;
use day7::Day7;
use day8::Day8;
use day9::Day9;

const ALL_CURRENT_DAYS: [&str; 12] = [
    "day1", "day2", "day3", "day4", "day5", "day6", "day7", "day8", "day9", "day10", "day11",
    "day12",
];

fn run_day_if_enabled(day: Box<dyn AOCCalculator>, name: &str, allowed_days: &[String]) {
    if !allowed_days.contains(&name.to_string()) {
        return;
    }
    day.print_results(name)
}

fn get_allowed_days() -> Vec<String> {
    if std::env::args().len() == 1 {
        ALL_CURRENT_DAYS.iter().map(|s| s.to_string()).collect()
    } else {
        let args: Vec<String> = std::env::args().collect();
        args[1..args.len()].to_vec()
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let allowed_days = get_allowed_days();
    run_day_if_enabled(
        Box::new(Day01::new("day1/data/input_data.txt")?),
        "day1",
        &allowed_days,
    );
    run_day_if_enabled(
        Box::new(Day2::new("day2/data/input_data.txt")?),
        "day2",
        &allowed_days,
    );
    run_day_if_enabled(
        Box::new(Day3::new("day3/data/input_data.txt")?),
        "day3",
        &allowed_days,
    );
    run_day_if_enabled(
        Box::new(Day4::new("day4/data/input_data.txt")?),
        "day4",
        &allowed_days,
    );
    run_day_if_enabled(
        Box::new(Day5::new("day5/data/input_data.txt")?),
        "day5",
        &allowed_days,
    );
    run_day_if_enabled(
        Box::new(Day6::new("day6/data/input_data.txt")?),
        "day6",
        &allowed_days,
    );
    run_day_if_enabled(
        Box::new(Day7::new("day7/data/input_data.txt")?),
        "day7",
        &allowed_days,
    );
    run_day_if_enabled(
        Box::new(Day8::new("day8/data/input_data.txt")?),
        "day8",
        &allowed_days,
    );
    run_day_if_enabled(
        Box::new(Day9::new("day9/data/input_data.txt")?),
        "day9",
        &allowed_days,
    );
    run_day_if_enabled(
        Box::new(Day10::new("day10/data/input_data.txt")?),
        "day10",
        &allowed_days,
    );
    run_day_if_enabled(
        Box::new(Day11::new("day11/data/input_data.txt")?),
        "day11",
        &allowed_days,
    );
    run_day_if_enabled(
        Box::new(Day12::new("day12/data/input_data.txt")?),
        "day12",
        &allowed_days,
    );
    Ok(())
}
