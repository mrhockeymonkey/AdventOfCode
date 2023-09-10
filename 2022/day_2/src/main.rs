use std::str::FromStr;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let rounds = include_str!("input.txt")
        .lines()
        //.map(|line| line.parse::<Round>())
        .map(Round::from_str)
        .collect::<Result<Vec<_>, _>>()?;

    let total_score = rounds.iter().map(|r| dbg!(dbg!(r).your_score())).sum::<u32>();
    dbg!(total_score);

    assert_eq!(12526, total_score);

    Ok(())
}

#[derive(Debug, Copy, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors
}

impl Move {
    fn inherent_points(self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn beats(self, other: Move) -> bool {
        matches!(
            (self, other),
            (Self::Rock, Self::Scissors)
                | (Self::Paper, Self::Rock)
                | (Self::Scissors, Self::Paper)
        )
    }

    fn outcome(self, theirs: Move) -> Outcome {
        if self.beats(theirs) {
            Outcome::Win
        } else if theirs.beats(self) {
            Outcome::Loss
        } else {
            Outcome::Draw
        }
    }
}

impl TryFrom<char> for Move {
    type Error = color_eyre::Report;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Move::Rock),
            'B' => Ok(Move::Paper),
            'C' => Ok(Move::Scissors),
            _ => Err(color_eyre::eyre::eyre!("Not a valid move: {value:?}"))
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Outcome {
    Loss,
    Draw,
    Win,
}

impl Outcome {
    fn inherent_points(self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0
        }
    }

    fn required_move(self, theirs: Move) -> Move {
        match self {
            Outcome::Win => match theirs {
                Move::Rock => Move::Paper,
                Move::Paper => Move::Scissors,
                Move::Scissors => Move::Rock
            }
            Outcome::Draw => theirs,
            Outcome::Loss => match theirs {
                Move::Rock => Move::Scissors,
                Move::Paper => Move::Rock,
                Move::Scissors => Move::Paper
            }
        }
    }
}

impl TryFrom<char> for Outcome {
    type Error = color_eyre::Report;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(Outcome::Loss),
            'Y' => Ok(Outcome::Draw),
            'Z' => Ok(Outcome::Win),
            _ => Err(color_eyre::eyre::eyre!("Not a valid outcome: {value:?}"))
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Round {
    theirs: Move,
    yours: Move
}

impl Round {
    fn outcome(self) -> Outcome {
        self.yours.outcome(self.theirs)
    }

    fn your_score(self) -> u32 {
        self.yours.inherent_points() + self.outcome().inherent_points()
    }
}

impl FromStr for Round {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let (Some(theirs), Some(' '), Some(outcome), None) = (chars.next(), chars.next(), chars.next(), chars.next()) else {
            return Err(color_eyre::eyre::eyre!("expected '<theirs> <yours>', but got: '{s}'"))
        };

        let theirs = Move::try_from(theirs)?;
        let outcome = Outcome::try_from(outcome)?;
        let yours = outcome.required_move(theirs);

        Ok(Self{
            theirs,
            yours,
        })
    }
}