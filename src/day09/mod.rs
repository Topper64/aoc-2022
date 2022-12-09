use std::collections::HashMap;

#[derive(Debug, Default, PartialEq)]
struct Point {
    pub x: i32,
    pub y: i32,
    pub tail: Option<Box<Point>>,
}

impl Point {
    pub fn new(length: usize) -> Self {
        let tail = match length {
            0 => None,
            _ => Some(Box::new(Point::new(length - 1))),
        };
        Point {
            tail,
            ..Default::default()
        }
    }

    pub fn last(&self) -> &Self {
        match self.tail {
            Some(ref tail) => tail.last(),
            None => self,
        }
    }
}

impl Eq for Point {}

impl std::ops::AddAssign<(i32, i32)> for Point {
    fn add_assign(&mut self, (dx, dy): (i32, i32)) {
        self.x += dx;
        self.y += dy;
        if let Some(tail) = &mut self.tail {
            let dx = self.x - tail.x;
            let dy = self.y - tail.y;
            let d = dx.abs().max(dy.abs()); // ∞-norm
            if d >= 2 {
                **tail += (dx.signum(), dy.signum());
            }
        }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let tail = match &self.tail {
            Some(tail) => format!(" {}", tail),
            None => String::from(""),
        };
        write!(f, "({}, {}){}", self.x, self.y, tail)
    }
}

fn simulate(input: &str, length: usize) -> usize {
    let mut grid = HashMap::new();
    let mut rope = Point::new(length);
    for (dir, count) in input.lines().map(|s| s.split_once(' ').unwrap()) {
        for _ in 0..count.parse().unwrap() {
            // Move head as intructed
            rope += match dir {
                "L" => (-1, 0),
                "R" => (1, 0),
                "U" => (0, -1),
                "D" => (0, 1),
                _ => panic!("invalid direction"),
            };
            // Track where the tail has been
            let tail = rope.last();
            *grid.entry((tail.x, tail.y)).or_insert(0) += 1;
        }
    }
    grid.len()
}

fn part1(input: &str) -> usize {
    simulate(input, 1)
}

fn part2(input: &str) -> usize {
    simulate(input, 9)
}

pub fn main() {
    let input = include_str!("input.txt");
    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("test.txt")), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("test_long.txt")), 36);
    }
}
