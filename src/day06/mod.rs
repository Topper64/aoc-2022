use std::collections::{HashSet, VecDeque};

fn part1(input: &str) -> usize {
    let mut buffer: VecDeque<_> = VecDeque::new();
    for (c, i) in input.chars().zip(1..) {
        buffer.push_back(c);
        if buffer.len() >= 4 {
            if buffer.len() > 4 {
                buffer.pop_front();
            }
            if buffer.iter().collect::<HashSet<_>>().len() == 4 {
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
