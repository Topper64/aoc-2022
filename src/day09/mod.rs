use std::collections::HashMap;

fn part1(input: &str) -> usize {
    let mut grid = HashMap::new();
    let mut head = (0, 0);
    let mut tail = (0, 0);
    for (dir, count) in input.lines().map(|s| s.split_once(' ').unwrap()) {
        for _ in 0..count.parse().unwrap() {
            // Move head as intructed
            match dir {
                "L" => head.0 -= 1,
                "R" => head.0 += 1,
                "U" => head.1 -= 1,
                "D" => head.1 += 1,
                _ => panic!("invalid direction"),
            };
            // Move tail to catch up
            let dx = tail.0 - head.0;
            let dy = tail.1 - head.1;
            match (dx, dy) {
                (2, -1..=1) => {tail.0 -= 1; tail.1 -= dy;},
                (-2, -1..=1) => {tail.0 += 1; tail.1 -= dy;},
                (-1..=1, 2) => {tail.0 -= dx; tail.1 -= 1;},
                (-1..=1, -2) => {tail.0 -= dx; tail.1 += 1;},
                _ => (),
            }
            // Track where the tail has been
            *grid.entry(tail).or_insert(0) += 1;
        }
    }
    grid.len()
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
        assert_eq!(part1(INPUT), 13);
    }
}
