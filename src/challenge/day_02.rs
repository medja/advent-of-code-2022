use num_enum::{FromPrimitive, IntoPrimitive};

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let result = input
        .iter()
        .map(|game| {
            let bytes = game.as_bytes();
            let opponent = Hand::from(bytes[0]);
            let player = Hand::from(bytes[2]);
            compute_outcome(opponent, player).score() + player.score()
        })
        .sum::<usize>();

    Ok(result)
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let result = input
        .iter()
        .map(|game| {
            let bytes = game.as_bytes();
            let opponent = Hand::from(bytes[0]);
            let outcome = Outcome::from(bytes[2]);
            outcome.score() + compute_hand(opponent, outcome).score()
        })
        .sum::<usize>();

    Ok(result)
}

#[derive(FromPrimitive, IntoPrimitive, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[repr(usize)]
enum Hand {
    #[default]
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn score(self) -> usize {
        usize::from(self) + 1
    }
}

impl From<u8> for Hand {
    fn from(value: u8) -> Self {
        match value {
            b'A' => Hand::Rock,
            b'B' => Hand::Paper,
            b'C' => Hand::Scissors,
            b'X' => Hand::Rock,
            b'Y' => Hand::Paper,
            b'Z' => Hand::Scissors,
            _ => unreachable!(),
        }
    }
}

#[derive(FromPrimitive, IntoPrimitive, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[repr(usize)]
enum Outcome {
    #[default]
    Draw,
    Win,
    Lose,
}

impl Outcome {
    fn score(self) -> usize {
        match self {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

impl From<u8> for Outcome {
    fn from(value: u8) -> Self {
        match value {
            b'X' => Outcome::Lose,
            b'Y' => Outcome::Draw,
            b'Z' => Outcome::Win,
            _ => unreachable!(),
        }
    }
}

fn compute_outcome(opponent: Hand, player: Hand) -> Outcome {
    let opponent_index = usize::from(opponent);
    let player_index = usize::from(player);

    if (opponent_index + 1) % 3 == player_index {
        Outcome::Win
    } else if opponent_index == player_index {
        Outcome::Draw
    } else {
        Outcome::Lose
    }
}

fn compute_hand(opponent: Hand, outcome: Outcome) -> Hand {
    ((usize::from(opponent) + usize::from(outcome)) % 3).into()
}
