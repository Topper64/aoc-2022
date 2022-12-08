fn parse_input(input: &str) -> Vec<Vec<u32>> {
    let mut forest = Vec::new();
    for line in input.lines() {
        forest.push(
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>(),
        );
    }

    forest
}

fn part1(input: &str) -> usize {
    let forest = parse_input(input);
    let mut count = 0;
    for (i, row) in forest.iter().enumerate() {
        for (j, tree) in row.iter().enumerate() {
            let left = row[..j].iter().all(|t| t < tree);
            let right = row[j + 1..].iter().all(|t| t < tree);
            let top = forest[..i].iter().all(|r| r[j] < *tree);
            let bottom = forest[i + 1..].iter().all(|r| r[j] < *tree);

            if left || right || top || bottom {
                count += 1;
            }
        }
    }

    count
}

fn part2(input: &str) -> usize {
    let forest = parse_input(input);
    let h = forest.len();
    let mut best = 0;
    for (i, row) in forest.iter().enumerate() {
        let w = row.len();
        for (j, tree) in row.iter().enumerate() {
            let left = row[..j].iter().rev().take_while(|t| t < &tree).count() + 1;
            let right = row[j + 1..].iter().take_while(|t| t < &tree).count() + 1;
            let top = forest[..i]
                .iter()
                .rev()
                .take_while(|r| r[j] < *tree)
                .count()
                + 1;
            let bottom = forest[i + 1..].iter().take_while(|r| r[j] < *tree).count() + 1;

            let score = left.min(j) * right.min(w - j - 1) * top.min(i) * bottom.min(h - i - 1);
            if score > best {
                best = score
            }
        }
    }

    best
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
    fn test_part1() {
        assert_eq!(part1(INPUT), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 8);
    }
}
