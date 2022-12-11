use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
enum ParseError {
    Op,
    Items,
    Test,
    Target,
}

#[derive(Debug)]
enum Op {
    Add,
    Mul,
}

impl FromStr for Op {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Op::Add),
            "*" => Ok(Op::Mul),
            _ => Err(ParseError::Op),
        }
    }
}

// A simple binary operation between two terms. A `None` term means "the old value"
#[derive(Debug)]
struct Update {
    left: Option<usize>,
    right: Option<usize>,
    op: Op,
}

impl Update {
    pub fn call(&self, old: usize) -> usize {
        let left = self.left.unwrap_or(old);
        let right = self.right.unwrap_or(old);
        match self.op {
            Op::Add => left + right,
            Op::Mul => left * right,
        }
    }
}

impl FromStr for Update {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();
        if words.next() != Some("new") {
            return Err(ParseError::Op);
        }
        if words.next() != Some("=") {
            return Err(ParseError::Op);
        }

        let left = words.next().and_then(|w| w.parse().ok());
        let op = words.next().ok_or(ParseError::Op).and_then(|w| w.parse())?;
        let right = words.next().and_then(|w| w.parse().ok());
        Ok(Self { left, right, op })
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    update: Update,
    test: usize,
    target: (usize, usize),
    count: usize,
}

impl Monkey {
    pub fn turn(&mut self, worry: usize) -> HashMap<usize, Vec<usize>> {
        let mut result = HashMap::new();
        for item in self.items.drain(..) {
            self.count += 1;
            let item = self.update.call(item) / worry;
            let target = match item % self.test == 0 {
                true => self.target.0,
                false => self.target.1,
            };
            result
                .entry(target)
                .or_insert_with(|| Vec::new())
                .push(item);
        }
        result
    }
}

impl FromStr for Monkey {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut items = None;
        let mut update = Err(ParseError::Op);
        let mut test = Err(ParseError::Test);
        let mut target = (Err(ParseError::Target), Err(ParseError::Target));

        for line in s.lines().map(|s| s.trim_start()) {
            match line.split_once(": ") {
                None => (), // This is just the first "Monkey i:" line
                Some(("Starting items", rest)) => {
                    items = Some(rest.split(", ").map(|s| s.parse().unwrap()).collect());
                }
                Some(("Operation", rest)) => {
                    update = rest.parse().map_err(|_| ParseError::Op);
                }
                Some(("Test", rest)) => {
                    // Assumes the first few words are "divisible by"
                    test = rest
                        .split_whitespace()
                        .next_back()
                        .ok_or(ParseError::Test)
                        .and_then(|s| s.parse().map_err(|_| ParseError::Test));
                }
                Some(("If true", rest)) => {
                    // Assumes the first few words are "throw to monkey"
                    target.0 = rest
                        .split_whitespace()
                        .next_back()
                        .ok_or(ParseError::Target)
                        .and_then(|s| s.parse().map_err(|_| ParseError::Target));
                }
                Some(("If false", rest)) => {
                    // Assumes the first few words are "throw to monkey"
                    target.1 = rest
                        .split_whitespace()
                        .next_back()
                        .ok_or(ParseError::Target)
                        .and_then(|s| s.parse().map_err(|_| ParseError::Target));
                }
                _ => (),
            };
        }

        let items = items.ok_or(ParseError::Items)?;
        Ok(Self {
            items,
            update: update?,
            test: test?,
            target: (target.0?, target.1?),
            count: 0,
        })
    }
}

#[derive(Debug)]
struct Monkeys(Vec<Monkey>);

impl Monkeys {
    pub fn rounds(&mut self, n: usize, worry: usize) -> usize {
        // We will work mod m
        let m: usize = self.0.iter().map(|monkey| monkey.test).product();

        for _ in 0..n {
            self.round(worry);

            // Keep numbers in range
            for monkey in self.0.iter_mut() {
                for item in monkey.items.iter_mut() {
                    *item %= m;
                }
            }
        }

        self.business()
    }

    pub fn round(&mut self, worry: usize) {
        for i in 0..self.0.len() {
            let throws = self.0[i].turn(worry);
            for (target, items) in throws.iter() {
                self.0[*target].items.extend(items);
            }
        }
    }

    pub fn business(&self) -> usize {
        let mut counts: Vec<_> = self.0.iter().map(|monkey| monkey.count).collect();
        counts.sort();
        counts.reverse();
        counts[0] * counts[1]
    }
}

impl FromStr for Monkeys {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.split("\n\n")
                .map(|s| s.parse().expect("could not parse monkey"))
                .collect(),
        ))
    }
}

fn part1(input: &str) -> usize {
    let mut monkeys: Monkeys = input.parse().expect("could not parse monkeys");
    monkeys.rounds(20, 3)
}

fn part2(input: &str) -> usize {
    let mut monkeys: Monkeys = input.parse().expect("could not parse monkeys");
    monkeys.rounds(10000, 1)
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
    fn test_round() {
        let mut monkeys: Monkeys = INPUT.parse().expect("could not parse monkeys");
        monkeys.round(3);
        assert_eq!(monkeys.0[0].items, vec![20, 23, 27, 26]);
        assert_eq!(monkeys.0[1].items, vec![2080, 25, 167, 207, 401, 1046]);
        assert_eq!(monkeys.0[2].items, Vec::new());
        assert_eq!(monkeys.0[3].items, Vec::new());
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 10605);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 2713310158);
    }
}
