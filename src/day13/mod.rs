use std::str::FromStr;

#[derive(Debug)]
struct ParseError;

#[derive(Debug)]
enum Packet {
    Int(i32),
    List(Vec<Packet>),
}

impl FromStr for Packet {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(s) = s.strip_prefix('[') {
            let s = s.strip_suffix(']').ok_or(ParseError)?;
            let mut packets = Vec::new();
            let mut parts = Vec::new();
            let mut depth = 0;
            for part in s.split(',') {
                if part.starts_with('[') {
                    // Start of a list
                    if part.ends_with(']') {
                        // But also the end of a list
                        packets.push(part.parse()?);
                    } else {
                        depth += 1;
                        parts.push(part);
                    }
                } else if part.ends_with(']') {
                    // End of a list
                    if !parts.is_empty() {
                        parts.push(",");
                    }
                    parts.push(part);
                    depth -= 1;
                    if depth == 0 {
                        // Reached the matching bracket at the outermost level
                        packets.push(parts.drain(..).collect::<String>().parse()?);
                    }
                } else if depth == 0 {
                    // Not a list, so must be a number or nothing at all
                    if !part.is_empty() {
                        packets.push(part.parse()?);
                    }
                } else {
                    // Must be somewhere within some brackets, so just save for later
                    if !parts.is_empty() {
                        // Add separating comma back in
                        parts.push(",");
                    }
                    parts.push(part);
                }
            }
            Ok(Packet::List(packets))
        } else {
            match s.parse() {
                Ok(n) => Ok(Packet::Int(n)),
                Err(_) => Err(ParseError),
            }
        }
    }
}

fn part1(input: &str) {
    for line in input.lines() {
        println!("{:?}", line.parse::<Packet>());
    }
}

pub fn main() {
    let input = include_str!("test.txt");
    part1(input);
}
