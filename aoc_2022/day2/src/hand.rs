#[derive(PartialEq, Eq, Debug)]
pub enum Hand {
    Rock,
    Paper,
    Scissors,
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

    pub fn calculate_score_vs(&self, other: &Hand) -> usize {
        if self.is_winner(other) {
            6 + self.get_score()
        } else if self.is_draw(other) {
            3 + self.get_score()
        } else {
            self.get_score()
        }
    }

    pub fn calculate_score_vs_day_b(&self, other: &Hand) -> usize {
        // rock means lose
        // paper means draw
        // scissors means win
        //
        match self {
            Hand::Rock => other.get_score_of_loser(),
            Hand::Paper => 3 + other.get_score(),
            Hand::Scissors => 6 + other.get_score_of_winner(),
        }
    }

    fn get_score_of_winner(&self) -> usize {
        match self {
            Hand::Rock => Hand::Paper.get_score(),
            Hand::Paper => Hand::Scissors.get_score(),
            Hand::Scissors => Hand::Rock.get_score(),
        }
    }

    fn get_score_of_loser(&self) -> usize {
        match self {
            Hand::Rock => Hand::Scissors.get_score(),
            Hand::Paper => Hand::Rock.get_score(),
            Hand::Scissors => Hand::Paper.get_score(),
        }
    }
}
