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

impl LanternFishState {
    fn new(vent_layout_input_str: &str) -> LanternFishState {
        day6_parser::parse(vent_layout_input_str).unwrap()
    }

    fn get_next_state(self: &LanternFishState) -> LanternFishState {
        let mut ret = [0; 9];
        for (i, fish_count) in self.lantern_count_by_age[1..].iter().enumerate() {
            ret[i] = *fish_count;
        }
        ret[6] += self.lantern_count_by_age[0];
        ret[8] = self.lantern_count_by_age[0];
        LanternFishState {
            lantern_count_by_age: ret,
        }
    }

    fn get_after_n_iterations(self: &LanternFishState, n: usize) -> LanternFishState {
        let mut ret = self.clone();
        for _ in 0..n {
            ret = ret.get_next_state();
        }
        ret
    }

    fn count(self: &LanternFishState) -> usize {
        return self.lantern_count_by_age.iter().sum();
    }

    fn calculate_day_a(self: &LanternFishState) -> usize {
        self.get_after_n_iterations(80).count()
    }

    fn calculate_day_b(self: &LanternFishState) -> usize {
        self.get_after_n_iterations(256).count()
    }
}

fn main() {
    let lantern_state = LanternFishState::new(include_str!("../input_data.txt"));
    let day_a = lantern_state.calculate_day_a();
    println!("Day a result: {}", day_a);
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
        let ret = lantern_state.get_next_state();
        assert_eq!(ret.lantern_count_by_age, [1, 1, 2, 1, 0, 0, 0, 0, 0]);
        let ret = ret.get_next_state();
        assert_eq!(ret.lantern_count_by_age, [1, 2, 1, 0, 0, 0, 1, 0, 1]);
        let ret = ret.get_next_state();
        assert_eq!(ret.lantern_count_by_age, [2, 1, 0, 0, 0, 1, 1, 1, 1]);
        let ret = ret.get_next_state();
        assert_eq!(ret.lantern_count_by_age, [1, 0, 0, 0, 1, 1, 3, 1, 2]);
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
