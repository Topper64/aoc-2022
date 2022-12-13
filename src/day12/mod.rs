use std::collections::HashSet;

fn walk<F>(input: &str, from: F) -> usize
where
    F: Fn(char) -> bool,
{
    // Parse input
    let mut changed = HashSet::new();
    let mut target = None;
    let heights: Vec<Vec<_>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    'E' => {
                        target = Some((x, y));
                        25
                    }
                    _ if from(c) => {
                        changed.insert((x, y));
                        0
                    }
                    _ => (c as i32) - 97,
                })
                .collect()
        })
        .collect();
    let target = target.expect("cannot parse input");

    let height = heights.len();
    let width = heights[0].len();
    let mut steps = vec![vec![width * height; width]; height];
    let mut step = 0;
    while !changed.is_empty() {
        let mut to_check = HashSet::new();
        for (x, y) in changed.drain() {
            // Record number of steps taken to get here
            steps[y][x] = step;

            // Find all neighbouring cells
            let mut neighbours = Vec::new();
            if x < width - 1 {
                neighbours.push((x + 1, y));
            }
            if x > 0 {
                neighbours.push((x - 1, y));
            }
            if y < height - 1 {
                neighbours.push((x, y + 1));
            }
            if y > 0 {
                neighbours.push((x, y - 1));
            }

            // Include the ones with a close enough height in the next iteration
            let h = heights[y][x] + 1;
            to_check.extend(
                neighbours
                    .iter()
                    .filter(|(x, y)| heights[*y][*x] <= h && step < steps[*y][*x]),
            );
        }

        step += 1;
        changed = to_check;
    }

    steps[target.1][target.0]
}

fn part1(input: &str) -> usize {
    walk(input, |c| c == 'S')
}

fn part2(input: &str) -> usize {
    walk(input, |c| c == 'S' || c == 'a')
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
        assert_eq!(part1(INPUT), 31);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 29);
    }
}
