use std::collections::HashSet;

fn priority(c: char) -> i32 {
    match c {
        'a'..='z' => c as i32 - 96,
        'A'..='Z' => c as i32 - 38,
        _ => panic!("unexpected character"),
    }
}

fn find_duplicate<It>(mut bags: Vec<It>) -> Option<char>
where
    It: Iterator<Item = char>,
{
    let mut result: Option<HashSet<_>> = None;
    for bag in bags.iter_mut().map(|b| b.collect()) {
        result = match result {
            None => Some(bag),
            Some(so_far) => Some(so_far.intersection(&bag).map(|c| *c).collect()),
        }
    }
    match result {
        Some(result) if result.len() == 1 => Some(*result.iter().next().unwrap()),
        _ => None,
    }
}

fn search_bag(items: &str) -> char {
    let len = items.len();
    let left = items.get(..len / 2).unwrap().chars();
    let right = items.get(len / 2..).unwrap().chars();
    find_duplicate(vec![left, right]).unwrap()
}

fn part1(input: &str) -> i32 {
    input.lines().map(search_bag).map(priority).sum()
}

fn part2(input: &str) -> i32 {
    let mut lines = input.lines();
    let mut result = 0;
    loop {
        match find_duplicate(lines.by_ref().take(3).map(|e| e.chars()).collect()) {
            Some(sticker) => result += priority(sticker),
            None => break result,
        }
    }
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
    fn test_priority() {
        assert_eq!(priority('p'), 16);
        assert_eq!(priority('P'), 42);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 157);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 70);
    }
}
