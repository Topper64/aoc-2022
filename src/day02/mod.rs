#[derive(Debug, PartialEq)]
enum Rps {
    Rock,
    Paper,
    Scissors,
}

impl Rps {
    fn value(&self) -> i32 {
        match self {
            Rps::Rock => 1,
            Rps::Paper => 2,
            Rps::Scissors => 3,
        }
    }

    pub fn plays(&self, other: &Rps) -> i32 {
        let score = match (self, other) {
            (Rps::Rock, Rps::Paper) => 0,
            (Rps::Paper, Rps::Scissors) => 0,
            (Rps::Scissors, Rps::Rock) => 0,
            (Rps::Rock, Rps::Scissors) => 6,
            (Rps::Paper, Rps::Rock) => 6,
            (Rps::Scissors, Rps::Paper) => 6,
            _ => 3,
        };
        score + self.value()
    }
}

fn parse(line: &str) -> (Rps, Rps) {
    let (left, right) = line.split_once(" ").expect("invalid input");
    let left = match left {
        "A" => Rps::Rock,
        "B" => Rps::Paper,
        "C" => Rps::Scissors,
        _ => panic!("invalid input"),
    };
    let right = match right {
        "X" => Rps::Rock,
        "Y" => Rps::Paper,
        "Z" => Rps::Scissors,
        _ => panic!("invalid input"),
    };
    (left, right)
}

fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(parse)
        .map(|(them, you)| you.plays(&them))
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
