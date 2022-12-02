#[derive(Debug)]
enum Throw {
    Rock,
    Paper,
    Scissors,
}

impl Throw {
    fn shape(self: &Self) -> u32 {
        match self {
            Throw::Rock => 1,
            Throw::Paper => 2,
            Throw::Scissors => 3,
        }
    }
    fn score(self: &Self, other: &Self) -> u32 {
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

#[derive(Debug)]
struct Round {
    elve: Throw,
    you: Throw,
}

impl Round {
    fn parse_from_line(line: &str) -> Self {
        let throws = line.split(" ").collect::<Vec<&str>>();
        println!("{:?}", throws);

        let elve = match throws[0] {
            "A" => Throw::Rock,
            "B" => Throw::Paper,
            "C" => Throw::Scissors,
            _ => panic!("Elve input is not valid!"),
        };
        let you = match throws[1] {
            "X" => Throw::Rock,
            "Y" => Throw::Paper,
            "Z" => Throw::Scissors,
            _ => panic!("Your input is not valid!"),
        };
        Self { elve, you }
    }

    fn score(self: &Self) -> u32 {
        println!("{:?}", self);
        self.you.score(&self.elve)
    }
}

fn main() {
    println!("Score: {}", Round::parse_from_line("A Y").score());
    println!("Score: {}", Round::parse_from_line("B X").score());
    println!("Score: {}", Round::parse_from_line("C Z").score());
}
