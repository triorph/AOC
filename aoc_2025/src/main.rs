use aoc_helpers::AOCCalculator;
use color_eyre::eyre::Result;
// use day1::Day01;
// use day10::Day10;
// use day11::Day11;
// use day12::Day12;
// use day13::Day13;
// use day14::Day14;
// use day15::Day15;
// use day16::Day16;
// use day17::Day17;
// use day18::Day18;
// use day19::Day19;
// use day2::Day2;
// use day20::Day20;
// use day21::Day21;
// use day22::Day22;
// use day23::Day23;
// use day24::Day24;
// use day25::Day25;
// use day3::Day3;
// use day4::Day4;
// use day5::Day5;
// use day6::Day6;
// use day7::Day7;
// use day8::Day8;
// use day9::Day9;

const ALL_CURRENT_DAYS: [&str; 0] = [
    // "day1", "day2", "day3", "day4", "day5", "day6", "day7", "day8", "day9", "day10", "day11",
    // "day12", "day13", "day14", "day15", "day16", "day17", "day18", "day19", "day20", "day21",
    // "day22", "day23", "day24", "day25",
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
    // run_day_if_enabled(
    //     Box::new(Day01::new("day1/data/input_data.txt")?),
    //     "day1",
    //     &allowed_days,
    // );
    // run_day_if_enabled(
    //     Box::new(Day2::new("day2/data/input_data.txt")?),
    //     "day2",
    //     &allowed_days,
    // );
    // run_day_if_enabled(
    //     Box::new(Day3::new("day3/data/input_data.txt")?),
    //     "day3",
    //     &allowed_days,
    // );
    // run_day_if_enabled(
    //     Box::new(Day4::new("day4/data/input_data.txt")?),
    //     "day4",
    //     &allowed_days,
    // );
    // run_day_if_enabled(
    //     Box::new(Day5::new("day5/data/input_data.txt")?),
    //     "day5",
    //     &allowed_days,
    // );
    // run_day_if_enabled(
    //     Box::new(Day6::new("day6/data/input_data.txt")?),
    //     "day6",
    //     &allowed_days,
    // );
    // run_day_if_enabled(
    //     Box::new(Day7::new("day7/data/input_data.txt")?),
    //     "day7",
    //     &allowed_days,
    // );
    // run_day_if_enabled(
    //     Box::new(Day8::new("day8/data/input_data.txt")?),
    //     "day8",
    //     &allowed_days,
    // );
    // run_day_if_enabled(
    //     Box::new(Day9::new("day9/data/input_data.txt")?),
    //     "day9",
    //     &allowed_days,
    // );
    // run_day_if_enabled(
    //     Box::new(Day10::new("day10/data/input_data.txt")?),
    //     "day10",
    //     &allowed_days,
    // );
    // run_day_if_enabled(
    //     Box::new(Day11::new("day11/data/input_data.txt")?),
    //     "day11",
    //     &allowed_days,
    // );
    // run_day_if_enabled(
    //     Box::new(Day12::new("day12/data/input_data.txt")?),
    //     "day12",
    //     &allowed_days,
    // );
    // run_day_if_enabled(
    //     Box::new(Day13::new("day13/data/input_data.txt")?),
    //     "day13",
    //     &allowed_days,
    // );
    // Day14::new("day14/data/input_data.txt", 101, 103)?.print_results("day14");
    // run_day_if_enabled(
    //     Box::new(Day15::new("day15/data/input_data.txt")?),
    //     "day15",
    //     &allowed_days,
    // );
    // run_day_if_enabled(
    //     Box::new(Day16::new("day16/data/input_data.txt")?),
    //     "day16",
    //     &allowed_days,
    // );
    // run_day_if_enabled(
    //     Box::new(Day17::new("day17/data/input_data.txt")?),
    //     "day17",
    //     &allowed_days,
    // );
    // run_day_if_enabled(
    //     Box::new(Day18::new("day18/data/input_data.txt")?),
    //     "day18",
    //     &allowed_days,
    // );
    // run_day_if_enabled(
    //     Box::new(Day19::new("day19/data/input_data.txt")?),
    //     "day19",
    //     &allowed_days,
    // );
    // run_day_if_enabled(
    //     Box::new(Day20::new("day20/data/input_data.txt")?),
    //     "day20",
    //     &allowed_days,
    // );
    // run_day_if_enabled(
    //     Box::new(Day21::new("day21/data/input_data.txt")?),
    //     "day21",
    //     &allowed_days,
    // );
    // run_day_if_enabled(
    //     Box::new(Day22::new("day22/data/input_data.txt")?),
    //     "day22",
    //     &allowed_days,
    // );
    // run_day_if_enabled(
    //     Box::new(Day23::new("day23/data/input_data.txt")?),
    //     "day23",
    //     &allowed_days,
    // );
    // run_day_if_enabled(
    //     Box::new(Day24::new("day24/data/input_data.txt")?),
    //     "day24",
    //     &allowed_days,
    // );
    // run_day_if_enabled(
    //     Box::new(Day25::new("day25/data/input_data.txt")?),
    //     "day25",
    //     &allowed_days,
    // );
    Ok(())
}
