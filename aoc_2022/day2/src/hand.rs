#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Hand {
    Rock,
    Paper,
    Scissors,
}

pub enum Result {
    Win,
    Draw,
    Loss,
}

impl Hand {
    fn get_score(&self) -> usize {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }

    fn is_draw(&self, other: &Hand) -> bool {
        self == other
    }

    fn is_winner(&self, other: &Hand) -> bool {
        matches!(
            (self, other),
            (Hand::Rock, Hand::Scissors)
                | (Hand::Scissors, Hand::Paper)
                | (Hand::Paper, Hand::Rock)
        )
    }

    fn get_result_vs(&self, other: &Hand) -> Result {
        if self.is_winner(other) {
            Result::Win
        } else if self.is_draw(other) {
            Result::Draw
        } else {
            Result::Loss
        }
    }

    pub fn calculate_score_vs_day_a(&self, other: &Hand) -> usize {
        self.get_result_vs(other).get_score() + self.get_score()
    }

    fn to_result(self) -> Result {
        match self {
            // X was rock in day a, but loss in day b
            Hand::Rock => Result::Loss,
            // Y was paper in day a, but draw in day b
            Hand::Paper => Result::Draw,
            // Z was scissros in day a, but win in day b
            Hand::Scissors => Result::Win,
        }
    }

    pub fn calculate_score_vs_day_b(&self, other: &Hand) -> usize {
        let game_result = self.to_result();
        game_result.get_opponent(other).get_score() + game_result.get_score()
    }
}

impl Result {
    fn get_score(&self) -> usize {
        match self {
            Result::Win => 6,
            Result::Draw => 3,
            Result::Loss => 0,
        }
    }

    fn get_opponent(&self, us: &Hand) -> Hand {
        match (self, us) {
            (Result::Loss, Hand::Rock) => Hand::Scissors,
            (Result::Loss, Hand::Paper) => Hand::Rock,
            (Result::Loss, Hand::Scissors) => Hand::Paper,
            (Result::Draw, val) => *val,
            (Result::Win, Hand::Rock) => Hand::Paper,
            (Result::Win, Hand::Paper) => Hand::Scissors,
            (Result::Win, Hand::Scissors) => Hand::Rock,
        }
    }
}
