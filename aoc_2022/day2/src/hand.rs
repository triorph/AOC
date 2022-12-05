#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Hand {
    Rock,
    Paper,
    Scissors,
}

pub enum Outcome {
    Loss,
    Draw,
    Win,
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

    fn get_result_vs(&self, other: &Hand) -> Outcome {
        if self.is_winner(other) {
            Outcome::Win
        } else if self.is_draw(other) {
            Outcome::Draw
        } else {
            Outcome::Loss
        }
    }

    pub fn calculate_score_vs_day_a(&self, other: &Hand) -> usize {
        self.get_result_vs(other).get_score() + self.get_score()
    }

    fn to_outcome(self) -> Outcome {
        match self {
            // X was rock in day a, but loss in day b
            Hand::Rock => Outcome::Loss,
            // Y was paper in day a, but draw in day b
            Hand::Paper => Outcome::Draw,
            // Z was scissros in day a, but win in day b
            Hand::Scissors => Outcome::Win,
        }
    }

    pub fn calculate_score_vs_day_b(&self, other: &Hand) -> usize {
        let desired_outcome = self.to_outcome();
        other.calculate_their_hand(&desired_outcome).get_score() + desired_outcome.get_score()
    }

    fn calculate_their_hand(&self, their_outcome: &Outcome) -> Hand {
        match (their_outcome, self) {
            (Outcome::Loss, Hand::Rock) => Hand::Scissors,
            (Outcome::Loss, Hand::Paper) => Hand::Rock,
            (Outcome::Loss, Hand::Scissors) => Hand::Paper,
            (Outcome::Draw, val) => *val,
            (Outcome::Win, Hand::Rock) => Hand::Paper,
            (Outcome::Win, Hand::Paper) => Hand::Scissors,
            (Outcome::Win, Hand::Scissors) => Hand::Rock,
        }
    }
}

impl Outcome {
    fn get_score(&self) -> usize {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        }
    }
}
