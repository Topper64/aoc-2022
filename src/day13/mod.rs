use std::cmp::Ordering;
use std::iter;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct ParseError;

#[derive(Debug, PartialEq, Eq)]
enum Packet {
    Int(i32),
    List(Vec<Packet>),
}

impl FromStr for Packet {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "[]" {
            // Short circuit for empty list
            Ok(Packet::List(Vec::new()))
        } else if let Some(s) = s.strip_prefix('[') {
            // Non-empty list
            let s = s.strip_suffix(']').ok_or(ParseError)?;
            let mut packets = Vec::new();
            let mut parts = Vec::new();
            let mut depth = 0;
            for part in s.split(',') {
                // Add to buffer
                if !parts.is_empty() {
                    // Add separating comma back in
                    parts.push(",");
                }
                parts.push(part);

                // Recalculate bracket depth
                depth += part.matches('[').count();
                depth -= part.matches(']').count();

                // Parse if at the outermost level
                if depth == 0 {
                    packets.push(parts.drain(..).collect::<String>().parse()?);
                }
            }
            if parts.is_empty() {
                Ok(Packet::List(packets))
            } else {
                Err(ParseError)
            }
        } else {
            // Number
            match s.parse() {
                Ok(n) => Ok(Packet::Int(n)),
                Err(_) => Err(ParseError),
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Int(x), Packet::Int(y)) => x.cmp(y),
            (Packet::List(xs), Packet::List(ys)) => xs
                .iter()
                .zip(ys)
                .find_map(|(x, y)| match x.cmp(y) {
                    Ordering::Equal => None,
                    result => Some(result),
                })
                .unwrap_or_else(|| xs.len().cmp(&ys.len())),
            (Packet::List(_), Packet::Int(y)) => self.cmp(&Packet::List(vec![Packet::Int(*y)])),
            (Packet::Int(x), Packet::List(_)) => Packet::List(vec![Packet::Int(*x)]).cmp(other),
        }
    }
}

fn part1(input: &str) -> usize {
    let mut packets = input
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<Packet>().unwrap());
    iter::from_fn(|| packets.next().zip(packets.next()))
        .zip(1..)
        .filter_map(|((l, r), i)| match l < r {
            true => Some(i),
            false => None,
        })
        .sum()
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
    fn test_parse_empty() {
        let input = "[]";
        let expected = Packet::List(vec![]);
        assert_eq!(input.parse(), Ok(expected));
    }

    #[test]
    fn test_parse_very_nested() {
        let input = "[[[10,4],7,[6,[1,2],8]]]";
        let expected = Packet::List(vec![Packet::List(vec![
            Packet::List(vec![Packet::Int(10), Packet::Int(4)]),
            Packet::Int(7),
            Packet::List(vec![
                Packet::Int(6),
                Packet::List(vec![Packet::Int(1), Packet::Int(2)]),
                Packet::Int(8),
            ]),
        ])]);
        assert_eq!(input.parse(), Ok(expected));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 13);
    }
}
