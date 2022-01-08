extern crate peg;
fn main() {
    let movements: Vec<Movement> = include_str!("../input_data.txt")
        .lines()
        .map(movement_parser::parse)
        .map(Result::unwrap)
        .collect();
    let day_a = calculate_day_a(&movements[..]);
    println!("Day a results: {}", day_a);
    let day_b = calculate_day_b(&movements[..]);
    println!("Day b results: {}", day_b);
}
peg::parser! { grammar movement_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']+) {n.parse().unwrap()}

    rule forward() -> Movement
        = "forward " n:number() {
            Movement::Forward( n )
        }

    rule down() -> Movement
        = "down " n:number() {
            Movement::Down(  n )
        }

    rule up() -> Movement
        = "up " n:number() {
            Movement::Up( n )
        }

    pub rule parse() -> Movement
        = movement:(forward() / down() / up() ){
            movement
        }
}}

#[derive(PartialEq, Debug)]
struct SubmarineLocationDayA {
    depth: isize,
    forward: usize,
}

#[derive(PartialEq, Debug)]
struct SubmarineLocationDayB {
    depth: isize,
    forward: usize,
    aim: isize,
}

#[derive(PartialEq, Debug)]
pub enum Movement {
    Forward(usize),
    Down(usize),
    Up(usize),
}

fn move_location_day_a(
    location: SubmarineLocationDayA,
    movement: &Movement,
) -> SubmarineLocationDayA {
    match movement {
        Movement::Forward(val) => SubmarineLocationDayA {
            forward: location.forward + *val,
            ..location
        },
        Movement::Up(val) => SubmarineLocationDayA {
            depth: location.depth - *val as isize,
            ..location
        },
        Movement::Down(val) => SubmarineLocationDayA {
            depth: location.depth + *val as isize,
            ..location
        },
    }
}

fn move_location_day_b(
    location: SubmarineLocationDayB,
    movement: &Movement,
) -> SubmarineLocationDayB {
    match movement {
        Movement::Forward(val) => SubmarineLocationDayB {
            forward: location.forward + *val,
            depth: location.depth + *val as isize * location.aim,
            ..location
        },
        Movement::Up(val) => SubmarineLocationDayB {
            aim: location.aim - *val as isize,
            ..location
        },
        Movement::Down(val) => SubmarineLocationDayB {
            aim: location.aim + *val as isize,
            ..location
        },
    }
}
fn find_movement_results_day_a(movements: &[Movement]) -> SubmarineLocationDayA {
    let mut current_location = SubmarineLocationDayA {
        depth: 0,
        forward: 0,
    };
    for movement in movements {
        current_location = move_location_day_a(current_location, movement);
    }
    current_location
}

fn find_movement_results_day_b(movements: &[Movement]) -> SubmarineLocationDayB {
    let mut current_location = SubmarineLocationDayB {
        depth: 0,
        forward: 0,
        aim: 0,
    };
    for movement in movements {
        current_location = move_location_day_b(current_location, movement);
    }
    current_location
}

fn calculate_day_a(movements: &[Movement]) -> isize {
    let final_location = find_movement_results_day_a(movements);
    final_location.depth * final_location.forward as isize
}

fn calculate_day_b(movements: &[Movement]) -> isize {
    let final_location = find_movement_results_day_b(movements);
    final_location.depth * final_location.forward as isize
}

#[cfg(test)]
mod test {
    use crate::calculate_day_a;
    use crate::calculate_day_b;
    use crate::find_movement_results_day_a;
    use crate::find_movement_results_day_b;
    use crate::move_location_day_a;
    use crate::move_location_day_b;
    use crate::movement_parser;
    use crate::Movement;
    use crate::SubmarineLocationDayA;
    use crate::SubmarineLocationDayB;
    #[test]
    fn test_parser() {
        if let Ok(parsed_val) = movement_parser::parse("forward 12") {
            assert_eq!(parsed_val, Movement::Forward(12));
        } else {
            panic!("Did not parse forward correctly");
        }
        if let Ok(parsed_val) = movement_parser::parse("up 15") {
            assert_eq!(parsed_val, Movement::Up(15));
        } else {
            panic!("Did not parse up correctly");
        }
        if let Ok(parsed_val) = movement_parser::parse("down 223") {
            assert_eq!(parsed_val, Movement::Down(223));
        } else {
            panic!("Did not parse down correctly");
        }
    }

    #[test]
    fn test_day_a() {
        let movements: Vec<Movement> = include_str!("../test_data.txt")
            .lines()
            .map(movement_parser::parse)
            .map(Result::unwrap)
            .collect();
        let ret = calculate_day_a(&movements[..]);
        assert_eq!(ret, 150)
    }

    #[test]
    fn test_day_b() {
        let movements: Vec<Movement> = include_str!("../test_data.txt")
            .lines()
            .map(movement_parser::parse)
            .map(Result::unwrap)
            .collect();
        let ret = calculate_day_b(&movements[..]);
        assert_eq!(ret, 900)
    }

    #[test]
    fn test_final_location_day_a() {
        let movements: Vec<Movement> = include_str!("../test_data.txt")
            .lines()
            .map(movement_parser::parse)
            .map(Result::unwrap)
            .collect();
        let ret = find_movement_results_day_a(&movements[..]);
        assert_eq!(
            ret,
            SubmarineLocationDayA {
                depth: 10,
                forward: 15
            }
        )
    }

    #[test]
    fn test_movements_day_a() {
        let mut current_position = SubmarineLocationDayA {
            depth: 0,
            forward: 0,
        };
        for (movement, depth, forward) in [
            (Movement::Forward(5), 0, 5),
            (Movement::Down(5), 5, 5),
            (Movement::Forward(8), 5, 13),
            (Movement::Up(3), 2, 13),
            (Movement::Down(8), 10, 13),
            (Movement::Forward(2), 10, 15),
        ] {
            current_position = move_location_day_a(current_position, &movement);
            assert_eq!(current_position, SubmarineLocationDayA { depth, forward });
        }
    }

    #[test]
    fn test_final_location_day_b() {
        let movements: Vec<Movement> = include_str!("../test_data.txt")
            .lines()
            .map(movement_parser::parse)
            .map(Result::unwrap)
            .collect();
        let ret = find_movement_results_day_b(&movements[..]);
        assert_eq!(
            ret,
            SubmarineLocationDayB {
                depth: 60,
                forward: 15,
                aim: 10
            }
        )
    }

    #[test]
    fn test_movements_day_b() {
        let mut current_position = SubmarineLocationDayB {
            depth: 0,
            forward: 0,
            aim: 0,
        };
        for (movement, depth, forward, aim) in [
            (Movement::Forward(5), 0, 5, 0),
            (Movement::Down(5), 0, 5, 5),
            (Movement::Forward(8), 40, 13, 5),
            (Movement::Up(3), 40, 13, 2),
            (Movement::Down(8), 40, 13, 10),
            (Movement::Forward(2), 60, 15, 10),
        ] {
            current_position = move_location_day_b(current_position, &movement);
            assert_eq!(
                current_position,
                SubmarineLocationDayB {
                    depth,
                    forward,
                    aim
                }
            );
        }
    }
}
