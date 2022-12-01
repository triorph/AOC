fn main() {
    let values = include_str!("../input_data.txt")
        .lines()
        .map(|line| line.parse::<usize>())
        .flatten()
        .collect::<Vec<usize>>();
    println!("Day a result: {}", calculate_day_a(&values));
    println!("Day b result: {}", calculate_day_b(&values));
}

fn calculate_day_a(values: &[usize]) -> usize {
    values
        .iter()
        .map(|&mass| calculate_fuel_required_day_a(mass))
        .sum::<usize>()
}

fn calculate_day_b(values: &[usize]) -> usize {
    values
        .iter()
        .map(|&mass| calculate_fuel_required_day_b(mass))
        .sum::<usize>()
}

///Naively calculate the fuel required for a given mass
fn calculate_fuel_required_day_a(mass: usize) -> usize {
    if mass <= 8 {
        0
    } else {
        mass / 3 - 2
    }
}

///More intelligently calculate the fuel required for a mass, including the mass of the fuel itself
fn calculate_fuel_required_day_b(mass: usize) -> usize {
    let next_fuel = calculate_fuel_required_day_a(mass);
    if next_fuel == 0 {
        next_fuel
    } else {
        next_fuel + calculate_fuel_required_day_b(next_fuel)
    }
}

#[cfg(test)]
mod test {
    use crate::calculate_fuel_required_day_a;
    use crate::calculate_fuel_required_day_b;

    #[test]
    fn test_calculate_fuel_required_day_a() {
        assert_eq!(calculate_fuel_required_day_a(12), 2);
        assert_eq!(calculate_fuel_required_day_a(14), 2);
        assert_eq!(calculate_fuel_required_day_a(1969), 654);
        assert_eq!(calculate_fuel_required_day_a(100756), 33583);
        assert_eq!(calculate_fuel_required_day_a(3), 0);
    }

    #[test]
    fn test_calculate_fuel_required_day_b() {
        assert_eq!(calculate_fuel_required_day_b(12), 2);
        assert_eq!(calculate_fuel_required_day_b(14), 2);
        assert_eq!(calculate_fuel_required_day_b(1969), 966);
        assert_eq!(calculate_fuel_required_day_b(100756), 50346);
    }
}
