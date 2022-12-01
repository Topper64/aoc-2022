fn parse_input(input: &str) -> Vec<u32> {
    let mut lines = input.lines().peekable();
    let mut result: Vec<u32> = Vec::new();
    while lines.peek().is_some() {
        result.push((&mut lines).map_while(|s| s.parse::<u32>().ok()).sum());
    }
    result
}

fn part1(input: &str) -> u32 {
    *parse_input(input).iter().max().unwrap()
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
    fn test_parse_input() {
        assert_eq!(parse_input(INPUT), vec![6000, 4000, 11000, 24000, 10000]);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 24000);
    }
}
