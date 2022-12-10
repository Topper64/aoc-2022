use std::str::Lines;

struct Program<'a> {
    x: i32,
    dx: Option<i32>,
    lines: Lines<'a>,
}

impl<'a> Iterator for Program<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(x) = self.dx {
            self.dx = None;
            self.x += x;
            Some(self.x)
        } else if let Some(line) = self.lines.next() {
            if let Some(("addx", arg)) = line.split_once(' ') {
                self.dx = Some(arg.parse().unwrap());
            }
            Some(self.x)
        } else {
            None
        }
    }
}

impl<'a> From<&'a str> for Program<'a> {
    fn from(s: &'a str) -> Self {
        Self {
            x: 1,
            dx: Some(0),
            lines: s.lines(),
        }
    }
}

fn part1(input: &str) -> i32 {
    let mut total = 0;
    for (x, cycle) in Program::from(input).zip(1..) {
        // Add signal strength if passing the specified cycles
        if cycle % 40 == 20 {
            total += ((cycle / 40) * 40 + 20) as i32 * x;
        }
    }
    total
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
        assert_eq!(part1(INPUT), 13140);
    }
}
