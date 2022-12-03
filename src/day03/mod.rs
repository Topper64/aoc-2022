use std::collections::HashSet;

fn priority(c: char) -> i32 {
    match c {
        'a'..='z' => c as i32 - 96,
        'A'..='Z' => c as i32 - 38,
        _ => panic!("unexpected character"),
    }
}

fn find_duplicate(items: &str) -> char {
    let len = items.len();
    let mut items = items.chars();
    let left: HashSet<_> = items.by_ref().take(len / 2).collect();
    let right: HashSet<_> = items.collect();
    *left.intersection(&right).next().unwrap()
}

fn part1(input: &str) -> i32 {
    input.lines().map(find_duplicate).map(priority).sum()
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
    fn test_priority() {
        assert_eq!(priority('p'), 16);
        assert_eq!(priority('P'), 42);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 157);
    }
}
