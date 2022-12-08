fn part1(input: &str) -> usize {
    let mut forest = Vec::new();
    for line in input.lines() {
        forest.push(
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>(),
        );
    }

    let mut count = 0;
    for (i, row) in forest.iter().enumerate() {
        for (j, tree) in row.iter().enumerate() {
            let left = row.iter().take(j).all(|t| t < tree);
            let right = row.iter().skip(j + 1).all(|t| t < tree);
            let top = forest.iter().take(i).all(|r| r[j] < *tree);
            let bottom = forest.iter().skip(i + 1).all(|r| r[j] < *tree);

            if left || right || top || bottom {
                count += 1;
            }
        }
    }

    count
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
        assert_eq!(part1(INPUT), 21);
    }
}
