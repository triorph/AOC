extern crate peg;

pub struct Day21Setup {
    players: [Player; 2],
    dice: DiceRoll,
}

#[derive(Debug, Clone)]
struct Player {
    position: usize,
    score: usize,
}

struct DiceRoll(usize);

peg::parser! { grammar day21_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule player() -> Player
        = "Player " number() " starting position: " n:number() {
            Player{ position: n % 10, score: 0 }
        }
    pub rule parse() -> Day21Setup
        = players:player() **<2,2> "\n" "\n" * {
            Day21Setup {players: players.try_into().expect("Should be exactly 2 players"), dice: DiceRoll(0)}
        }
}}

impl DiceRoll {
    fn roll(&mut self) -> usize {
        let ret = self.0 % 100 + 1;
        self.0 += 1;
        ret
    }
}

impl Player {
    fn add_to_track(&mut self, val: usize) {
        self.position = (self.position + val) % 10;
        self.score += self.get_score_at_position();
    }

    fn get_score_at_position(&self) -> usize {
        if self.position == 0 {
            10
        } else {
            self.position
        }
    }

    fn has_won_a(&self) -> bool {
        self.score >= 1000
    }

    fn has_won_b(&self) -> bool {
        self.score >= 21
    }
}

impl Day21Setup {
    /// Generates a new Day21Setup object to be calculated upon
    ///
    /// Inputs: the full string contents of the input data set.
    ///
    /// Returns: A new Day21Setup object, with methods `calculate_day_a` and `calculate_day_b`
    /// available
    pub fn new(input_str: &str) -> Day21Setup {
        day21_parser::parse(input_str).unwrap()
    }

    fn get_player(&mut self, player_to_roll: bool) -> &mut Player {
        if player_to_roll {
            &mut self.players[1]
        } else {
            &mut self.players[0]
        }
    }

    fn run_roll_a(&mut self, player_to_roll: bool) {
        let dice_rolls = self.dice.roll() + self.dice.roll() + self.dice.roll();
        self.get_player(player_to_roll).add_to_track(dice_rolls);
    }

    fn run_roll_day_b(&mut self, player_to_roll: bool) -> [usize; 2] {
        let mut wins = [0, 0];
        let dice_rolls_and_count = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];
        let backup = self.get_player(player_to_roll).clone();
        for (dice_roll, count) in dice_rolls_and_count {
            let won = {
                let player = self.get_player(player_to_roll);
                player.add_to_track(dice_roll);
                player.has_won_b()
            };
            let found_wins = if won {
                self.get_wins_for_player(player_to_roll)
            } else {
                self.run_roll_day_b(!player_to_roll)
            };
            wins[0] += found_wins[0] * count;
            wins[1] += found_wins[1] * count;
            let player = self.get_player(player_to_roll);

            player.score = backup.score;
            player.position = backup.position;
        }
        wins
    }

    fn get_wins_for_player(&mut self, player_to_roll: bool) -> [usize; 2] {
        if player_to_roll {
            [0, 1]
        } else {
            [1, 0]
        }
    }

    fn run_rolls_until_winner_a(&mut self) {
        let mut player_to_roll = false;
        while self.players.iter().all(|player| !player.has_won_a()) {
            self.run_roll_a(player_to_roll);
            player_to_roll = !player_to_roll;
        }
    }

    fn get_losing_player(&self) -> &Player {
        if !self.players[0].has_won_a() {
            &self.players[0]
        } else {
            &self.players[1]
        }
    }

    fn run_rolls_until_winner_b(&mut self) -> [usize; 2] {
        self.run_roll_day_b(false)
    }

    /// Calculate the part a response
    pub fn calculate_day_a(&mut self) -> usize {
        self.run_rolls_until_winner_a();
        self.get_losing_player().score * self.dice.0
    }

    /// Calculate the part b response
    pub fn calculate_day_b(&mut self) -> usize {
        let wins = self.run_rolls_until_winner_b();
        println!("Wins are {:?}", wins);
        *wins.iter().max().expect("Should be a winner")
    }
}

#[cfg(test)]
mod test {
    use crate::Day21Setup;

    #[test]
    fn test_parse() {
        let _day21_setup = Day21Setup::new(include_str!("../test_data.txt"));
    }

    #[test]
    fn test_day_a() {
        let mut day21_setup = Day21Setup::new(include_str!("../test_data.txt"));
        assert_eq!(day21_setup.calculate_day_a(), 739785);
    }

    #[test]
    fn test_day_b() {
        let mut day21_setup = Day21Setup::new(include_str!("../test_data.txt"));
        assert_eq!(day21_setup.calculate_day_b(), 444356092776315);
    }
}
