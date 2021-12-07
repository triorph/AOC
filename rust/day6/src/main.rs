extern crate peg;

#[derive(Clone)]
pub struct LanternFishState {
    lantern_count_by_age: [usize; 9],
}

peg::parser! { grammar day6_parser() for str {
    rule lantern_age() -> usize
        = n:$(['0'..='8']) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    pub rule parse() -> LanternFishState
        = lantern_ages:lantern_age() ** ("," +) "\n" * {
            let mut ret = [0;9];
            for lantern_age in lantern_ages.iter() {
                ret[*lantern_age] += 1;
            }
            LanternFishState{ lantern_count_by_age: ret }
        }

}
}

struct LanternFishIterator {
    current_lantern: LanternFishState,
}

impl Iterator for LanternFishIterator {
    type Item = LanternFishState;
    fn next(self: &mut LanternFishIterator) -> Option<Self::Item> {
        let mut ret = [0; 9];
        for (i, fish_count) in self.current_lantern.lantern_count_by_age[1..]
            .iter()
            .enumerate()
        {
            ret[i] = *fish_count;
        }
        ret[6] += self.current_lantern.lantern_count_by_age[0];
        ret[8] = self.current_lantern.lantern_count_by_age[0];
        self.current_lantern = LanternFishState {
            lantern_count_by_age: ret,
        };
        Some(self.current_lantern.clone())
    }
}

impl LanternFishState {
    fn new(lantern_fish_state_input_str: &str) -> LanternFishState {
        day6_parser::parse(lantern_fish_state_input_str).unwrap()
    }

    fn get_after_n_iterations(self: &LanternFishState, n: usize) -> LanternFishState {
        self.iter().take(n).last().unwrap()
    }

    fn iter(self: &LanternFishState) -> LanternFishIterator {
        LanternFishIterator {
            current_lantern: self.clone(),
        }
    }

    fn calculate_day_a(self: &LanternFishState) -> usize {
        self.get_after_n_iterations(80).count()
    }

    fn calculate_day_b(self: &LanternFishState) -> usize {
        self.get_after_n_iterations(256).count()
    }
    fn count(self: &LanternFishState) -> usize {
        return self.lantern_count_by_age.iter().sum();
    }
}

fn main() {
    let lantern_state = LanternFishState::new(include_str!("../input_data.txt"));
    let day_a = lantern_state.calculate_day_a();
    println!("Day a result: {}", day_a);
    let lantern_state = LanternFishState::new(include_str!("../input_data.txt"));
    let day_b = lantern_state.calculate_day_b();
    println!("Day b result: {}", day_b);
}

#[cfg(test)]
mod test {
    use crate::LanternFishState;

    #[test]
    fn test_parse() {
        let lantern_state = LanternFishState::new(include_str!("../test_data.txt"));
        assert_eq!(
            lantern_state.lantern_count_by_age,
            [0, 1, 1, 2, 1, 0, 0, 0, 0]
        );
    }

    #[test]
    fn test_get_next_state() {
        let lantern_state = LanternFishState::new(include_str!("../test_data.txt"));
        let mut lantern_iter = lantern_state.iter();
        assert_eq!(
            lantern_iter.next().unwrap().lantern_count_by_age,
            [1, 1, 2, 1, 0, 0, 0, 0, 0]
        );
        assert_eq!(
            lantern_iter.next().unwrap().lantern_count_by_age,
            [1, 2, 1, 0, 0, 0, 1, 0, 1]
        );
        assert_eq!(
            lantern_iter.next().unwrap().lantern_count_by_age,
            [2, 1, 0, 0, 0, 1, 1, 1, 1]
        );
        assert_eq!(
            lantern_iter.next().unwrap().lantern_count_by_age,
            [1, 0, 0, 0, 1, 1, 3, 1, 2]
        );
    }

    #[test]
    fn test_num_iterations() {
        let lantern_state = LanternFishState::new(include_str!("../test_data.txt"));
        assert_eq!(lantern_state.get_after_n_iterations(18).count(), 26)
    }

    #[test]
    fn test_day_a() {
        let lantern_state = LanternFishState::new(include_str!("../test_data.txt"));
        assert_eq!(lantern_state.calculate_day_a(), 5934);
    }

    #[test]
    fn test_day_b() {
        let lantern_state = LanternFishState::new(include_str!("../test_data.txt"));
        assert_eq!(lantern_state.calculate_day_b(), 26984457539);
    }
}
