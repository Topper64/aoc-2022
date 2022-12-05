use std::collections::HashMap;
use std::str::Lines;

#[derive(Debug, PartialEq)]
struct Stacks {
    stacks: HashMap<char, Vec<char>>,
}

impl From<&mut Lines<'_>> for Stacks {
    fn from(lines: &mut Lines) -> Stacks {
        // Read everything - crates and the numbers below
        let mut stacks: Vec<Vec<char>> = Vec::new();
        for line in lines.take_while(|s| !s.is_empty()) {
            let items = line.chars().skip(1).step_by(4).map(|c| c);
            if stacks.is_empty() {
                stacks.extend(items.map(|c| vec![c]));
            } else {
                for (stack, item) in stacks.iter_mut().zip(items) {
                    stack.push(item);
                }
            }
        }

        // Extract the numbers from the (current) top of each stack
        let keys: Vec<_> = stacks.iter_mut().map(|v| v.pop().unwrap()).collect();

        // Flip stacks so the top crate is actually on top
        let stacks = stacks
            .into_iter()
            .map(|v| v.into_iter().rev().filter(|c| c != &' ').collect());

        Stacks {
            stacks: keys.into_iter().zip(stacks).collect(),
        }
    }
}

pub fn main() {}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("test.txt");

    #[test]
    fn test_parse_stacks() {
        let expected = HashMap::from([
            ('1', vec!['Z', 'N']),
            ('2', vec!['M', 'C', 'D']),
            ('3', vec!['P']),
        ]);
        assert_eq!(Stacks::from(&mut INPUT.lines()).stacks, expected);
    }
}
