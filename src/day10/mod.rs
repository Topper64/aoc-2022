fn part1(input: &str) -> i32 {
    let mut total = 0;
    let mut cycle = 0;
    let mut x = 1;
    for line in input.lines() {
        let (dc, dx) = match line.split_once(' ').unwrap_or_else(|| (line, "")) {
            ("noop", "") => (1, 0),
            ("addx", arg) => (2, arg.parse().unwrap()),
            _ => panic!("invalid input"),
        };

        // Add signal strength if passing the specified cycles
        let next_cycle = cycle + dc;
        if cycle % 40 < 20 && next_cycle % 40 >= 20 {
            total += ((cycle / 40) * 40 + 20) * x;
        }
        cycle = next_cycle;

        x += dx;
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
