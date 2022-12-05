use std::str::Lines;

#[derive(Debug, PartialEq)]
struct Stacks {
    stacks: Vec<Vec<char>>,
}

impl Stacks {
    pub fn tops(&self) -> String {
        self.stacks.iter().map(|s| s.last().unwrap()).collect()
    }

    pub fn rearrange(&mut self, count: usize, from: usize, to: usize) {
        for _ in 0..count {
            let item = self.stacks.get_mut(from).unwrap().pop().unwrap();
            self.stacks.get_mut(to).unwrap().push(item);
        }
    }
}

impl From<&mut Lines<'_>> for Stacks {
    fn from(lines: &mut Lines) -> Stacks {
        // Read everything - crates and the numbers below
        let mut stacks: Vec<Vec<char>> = Vec::new();
        for line in lines.take_while(|s| !s.is_empty()) {
            let items = line.chars().skip(1).step_by(4).map(|c| c);
            if stacks.is_empty() {
                stacks.extend(items.map(|c| vec![c]));
            } else {
                for (stack, item) in stacks.iter_mut().zip(items) {
                    stack.push(item);
                }
            }
        }

        // Flip stacks so the top crate is actually on top
        let stacks = stacks
            .into_iter()
            .map(|v| v.into_iter().rev().skip(1).filter(|c| c != &' ').collect());

        Stacks {
            stacks: stacks.collect(),
        }
    }
}

fn part1(input: &str) -> String {
    let mut lines = input.lines();
    let mut stacks = Stacks::from(&mut lines);

    for line in lines {
        let mut numbers = line
            .split_whitespace()
            .skip(1)
            .step_by(2)
            .map(|c| c.parse().unwrap());
        stacks.rearrange(
            numbers.next().unwrap(),
            numbers.next().unwrap() - 1,
            numbers.next().unwrap() - 1,
        );
    }

    stacks.tops()
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
    fn test_parse_stacks() {
        let expected = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
        assert_eq!(Stacks::from(&mut INPUT.lines()).stacks, expected);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), "CMZ");
    }
}
