use std::ops::{BitAnd, BitOr};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct InputError;

#[derive(Clone, Debug, PartialEq)]
enum Rps {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for Rps {
    type Err = InputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Rps::Rock),
            "B" | "Y" => Ok(Rps::Paper),
            "C" | "Z" => Ok(Rps::Scissors),
            _ => Err(InputError),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Outcome {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

impl FromStr for Outcome {
    type Err = InputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Loss),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(InputError),
        }
    }
}

impl BitOr for &Rps {
    type Output = Outcome;

    fn bitor(self, other: Self) -> Self::Output {
        match (self, other) {
            (Rps::Rock, Rps::Paper) => Outcome::Loss,
            (Rps::Paper, Rps::Scissors) => Outcome::Loss,
            (Rps::Scissors, Rps::Rock) => Outcome::Loss,
            (Rps::Rock, Rps::Scissors) => Outcome::Win,
            (Rps::Paper, Rps::Rock) => Outcome::Win,
            (Rps::Scissors, Rps::Paper) => Outcome::Win,
            _ => Outcome::Draw,
        }
    }
}

impl BitAnd<&Rps> for &Outcome {
    type Output = Rps;

    fn bitand(self, other: &Rps) -> Self::Output {
        match (self, other) {
            (Outcome::Draw, rps) => rps.clone(),
            (Outcome::Win, Rps::Rock) => Rps::Paper,
            (Outcome::Win, Rps::Paper) => Rps::Scissors,
            (Outcome::Win, Rps::Scissors) => Rps::Rock,
            (Outcome::Loss, Rps::Rock) => Rps::Scissors,
            (Outcome::Loss, Rps::Paper) => Rps::Rock,
            (Outcome::Loss, Rps::Scissors) => Rps::Paper,
        }
    }
}

fn parse<T: FromStr>(line: &str) -> (Rps, T) {
    let (left, right) = line.split_once(" ").expect("invalid input");
    let left = left.parse().expect("invalid input");
    let right = right.parse().ok().expect("invalid input");
    (left, right)
}

fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(parse)
        .map(|(them, you)| (&you | &them) as i32 + you as i32)
        .sum()
}

fn part2(input: &str) -> i32 {
    input
        .lines()
        .map(parse)
        .map(|(them, outcome)| (&outcome & &them) as i32 + outcome as i32)
        .sum()
}

pub fn main() {
    let input = include_str!("input.txt");
    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("test.txt");

    #[test]
    fn test_parse_moves() {
        assert_eq!(parse("A X"), (Rps::Rock, Rps::Rock));
    }

    #[test]
    fn test_parse_outcome() {
        assert_eq!(parse("A X"), (Rps::Rock, Outcome::Loss));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 15);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 12);
    }
}
