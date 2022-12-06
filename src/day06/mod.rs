use std::collections::{HashMap, VecDeque};

fn part1(input: &str) -> usize {
    let mut buffer: VecDeque<_> = VecDeque::new();
    let mut counts: HashMap<_, _> = HashMap::new();
    for (c, i) in input.chars().zip(1..) {
        buffer.push_back(c);
        *counts.entry(c).or_insert(0) += 1;
        if buffer.len() >= 4 {
            if buffer.len() > 4 {
                *counts.get_mut(&buffer.pop_front().unwrap()).unwrap() -= 1;
            }
            if counts.values().filter(|c| *c == &1).count() == 4 {
                return i;
            }
        }
    }
    input.len()
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
        assert_eq!(part1(INPUT), 7);
    }
}
