use std::collections::VecDeque;

#[derive(Debug, Default)]
struct Grid<T: Default> {
    pub left: i32,
    pub right: i32,
    pub top: i32,
    pub bottom: i32,
    pub cells: VecDeque<VecDeque<T>>,
}

impl<T: Default> Grid<T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn insert(&mut self, x: i32, y: i32, value: T) {
        // Update boundaries
        if self.cells.is_empty() {
            // Not added anything yet so can simply set them
            self.left = x;
            self.right = x + 1;
            self.top = y;
            self.bottom = y + 1;

            let mut row = VecDeque::new();
            row.push_back(value);
            self.cells.push_back(row);
        } else {
            if x < self.left || x >= self.right {
                // Extend width
                let left = x.min(self.left);
                let right = (x + 1).max(self.right);
                let width = (right - left) as usize;
                let offset = (self.left - x).max(0) as usize;
                self.left = left;
                self.right = right;
                for row in self.cells.iter_mut() {
                    row.resize_with(width, Default::default);
                    if offset > 0 {
                        row.rotate_right(offset);
                    }
                }
            }
            if y < self.top || y >= self.bottom {
                // Extend height
                let top = y.min(self.top);
                let bottom = (y + 1).max(self.bottom);
                let width = (self.right - self.left) as usize;
                let height = (bottom - top) as usize;
                let offset = (self.top - y).max(0) as usize;
                self.top = top;
                self.bottom = bottom;
                self.cells.resize_with(height, || {
                    let mut row = VecDeque::with_capacity(width);
                    row.resize_with(width, Default::default);
                    row
                });
                if offset > 0 {
                    self.cells.rotate_right(offset);
                }
            }

            self.cells[(y - self.top) as usize][(x - self.left) as usize] = value;
        }
    }

    pub fn get(&self, x: i32, y: i32) -> Option<&T> {
        if (self.left..self.right).contains(&x) && (self.top..self.bottom).contains(&y) {
            Some(&self.cells[(y - self.top) as usize][(x - self.left) as usize])
        } else {
            None
        }
    }
}

fn parse_input(input: &str) -> Grid<bool> {
    let mut grid = Grid::new();
    for line in input.lines() {
        line.split(" -> ")
            .map(|s| s.split_once(",").unwrap())
            .map(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()))
            .reduce(|(x0, y0), (x1, y1)| {
                if x0 == x1 {
                    let x = x0;
                    for y in y0.min(y1)..=y0.max(y1) {
                        grid.insert(x, y, true);
                    }
                } else if y0 == y1 {
                    let y = y0;
                    for x in x0.min(x1)..=x0.max(x1) {
                        grid.insert(x, y, true);
                    }
                }
                (x1, y1)
            });
    }

    // Ensure top is at 0
    grid.insert(500, 0, false);

    grid
}

fn simulate(mut grid: Grid<bool>) -> usize {
    let mut count = 0;
    loop {
        let mut x = 500;
        let mut y = 0;

        // Loop as long as it is possible to place the sand
        while grid.get(x, y) == Some(&false) {
            // Try to move down, down-left, and down-right in that order
            let options = vec![(x, y + 1), (x - 1, y + 1), (x + 1, y + 1)];
            let next = options
                .iter()
                .find(|(x, y)| grid.get(*x, *y) != Some(&true));
            (x, y) = match next {
                None => break,
                Some(next) => *next,
            };
        }

        if grid.get(x, y) == Some(&false) {
            // Can place
            grid.insert(x, y, true);
            count += 1;
        } else {
            // Either fell out of bounds or the spawn point was blocked
            break count;
        }
    }
}

fn part1(input: &str) -> usize {
    simulate(parse_input(input))
}

fn part2(input: &str) -> usize {
    let mut grid = parse_input(input);

    // Insert the floor.  Theoretically infinitely long but 2*height is sufficient.
    let bottom = grid.bottom + 1;
    let height = grid.bottom - grid.top + 2;
    let left = 500 - height;
    let right = 500 + height;
    for x in left..=right {
        grid.insert(x, bottom, true);
    }

    simulate(grid)
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
        assert_eq!(part1(INPUT), 24);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 93);
    }
}
