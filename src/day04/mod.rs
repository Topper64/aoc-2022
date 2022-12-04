use std::ops::Range;

trait Preorder<Rhs = Self> {
    fn preceeds(&self, other: &Rhs) -> bool;
}

impl<T: PartialOrd> Preorder for Range<T> {
    fn preceeds(&self, other: &Self) -> bool {
        other.start <= self.start && self.end <= other.end
    }
}

trait Connected {
    fn connected(&self, other: &Self) -> bool;
}

impl<T: PartialOrd> Connected for Range<T> {
    fn connected(&self, other: &Self) -> bool {
        other.start <= self.end && self.start <= other.end
    }
}

fn parse_line(line: &str) -> (Range<i32>, Range<i32>) {
    let (left, right) = line.split_once(",").unwrap();
    let (a, b) = left.split_once("-").unwrap();
    let (c, d) = right.split_once("-").unwrap();
    (
        a.parse().unwrap()..b.parse().unwrap(),
        c.parse().unwrap()..d.parse().unwrap(),
    )
}

fn process<F>(input: &str, func: F) -> usize
where
    F: FnMut((Range<i32>, Range<i32>)) -> bool,
{
    input
        .lines()
        .map(parse_line)
        .map(func)
        .filter(|x| *x)
        .count()
}

fn part1(input: &str) -> usize {
    process(input, |(a, b)| a.preceeds(&b) || b.preceeds(&a))
}

fn part2(input: &str) -> usize {
    process(input, |(a, b)| a.connected(&b) || b.connected(&a))
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
    fn test_range_preorder() {
        assert_eq!((1..2).preceeds(&(1..2)), true);
        assert_eq!((2..3).preceeds(&(1..4)), true);
        assert_eq!((1..4).preceeds(&(2..3)), false);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 4);
    }
}
