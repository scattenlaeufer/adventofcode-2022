enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Outcome {
    fn from_str(outcome_str: &str) -> Self {
        match outcome_str {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("Outcome {:?} not defined!", outcome_str),
        }
    }
}

#[derive(Clone)]
enum Throw {
    Rock,
    Paper,
    Scissors,
}

impl Throw {
    fn from_outcome(other: &Self, outcome: Outcome) -> Self {
        match outcome {
            Outcome::Lose => match other {
                Self::Rock => Self::Scissors,
                Self::Paper => Self::Rock,
                Self::Scissors => Self::Paper,
            },
            Outcome::Draw => other.clone(),
            Outcome::Win => match other {
                Self::Rock => Self::Paper,
                Self::Paper => Self::Scissors,
                Self::Scissors => Self::Rock,
            },
        }
    }

    fn shape(&self) -> u32 {
        match self {
            Throw::Rock => 1,
            Throw::Paper => 2,
            Throw::Scissors => 3,
        }
    }

    fn score(&self, other: &Self) -> u32 {
        self.shape()
            + match self {
                Throw::Rock => match other {
                    Throw::Rock => 3,
                    Throw::Paper => 0,
                    Throw::Scissors => 6,
                },
                Throw::Paper => match other {
                    Throw::Rock => 6,
                    Throw::Paper => 3,
                    Throw::Scissors => 0,
                },
                Throw::Scissors => match other {
                    Throw::Rock => 0,
                    Throw::Paper => 6,
                    Throw::Scissors => 3,
                },
            }
    }
}

pub struct Round {
    elve: Throw,
    you: Throw,
}

impl Round {
    pub fn parse_from_line(line: &str) -> Self {
        let throws = line.split(' ').collect::<Vec<&str>>();

        let elve = match throws[0] {
            "A" => Throw::Rock,
            "B" => Throw::Paper,
            "C" => Throw::Scissors,
            _ => panic!("Elve input is not valid!"),
        };
        let you = Throw::from_outcome(&elve, Outcome::from_str(throws[1]));
        Self { elve, you }
    }

    pub fn score(&self) -> u32 {
        self.you.score(&self.elve)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn score_test_plays() {
        assert_eq!(4, Round::parse_from_line("A Y").score());
        assert_eq!(1, Round::parse_from_line("B X").score());
        assert_eq!(7, Round::parse_from_line("C Z").score());
    }
}
