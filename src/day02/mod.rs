use std::ops::BitOr;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct InputError;

#[derive(Debug, PartialEq)]
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

fn parse(line: &str) -> (Rps, Rps) {
    let (left, right) = line.split_once(" ").expect("invalid input");
    let left = left.parse().expect("invalid input");
    let right = right.parse().expect("invalid input");
    (left, right)
}

fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(parse)
        .map(|(them, you)| (&you | &them) as i32 + you as i32)
        .sum()
}

pub fn main() {
    let input = include_str!("input.txt");
    println!("part 1: {}", part1(input));
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 15);
    }
}
