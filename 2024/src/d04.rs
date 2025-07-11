use crate::Solution;

pub struct Day04;

impl Solution for Day04 {
    type Part1 = u32;
    type Part2 = u32;

    fn day(&self) -> u32 {
        4
    }

    fn solve_p1(&self, input: &str) -> Self::Part1 {
        let grid = input
            .trim_end()
            .to_string()
            .split("\n")
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let mut total = 0;

        // look for XMAS in all directions in grid
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] != 'X' {
                    continue;
                }

                // check up
                if i > 2 && grid[i - 1][j] == 'M' && grid[i - 2][j] == 'A' && grid[i - 3][j] == 'S'
                {
                    total += 1;
                    continue;
                }

                // check down
                if i < grid.len() - 3
                    && grid[i + 1][j] == 'M'
                    && grid[i + 2][j] == 'A'
                    && grid[i + 3][j] == 'S'
                {
                    total += 1;
                    continue;
                }

                // check left
                if j > 2 && grid[i][j - 1] == 'M' && grid[i][j - 2] == 'A' && grid[i][j - 3] == 'S'
                {
                    total += 1;
                }

                // check right
                if j < grid[i].len() - 3
                    && grid[i][j + 1] == 'M'
                    && grid[i][j + 2] == 'A'
                    && grid[i][j + 3] == 'S'
                {
                    total += 1;
                    continue;
                }

                // check up-left
                if i > 2
                    && j > 2
                    && grid[i - 1][j - 1] == 'M'
                    && grid[i - 2][j - 2] == 'A'
                    && grid[i - 3][j - 3] == 'S'
                {
                    total += 1;
                    continue;
                }

                // check up-right
                if i > 2
                    && j < grid[i].len() - 3
                    && grid[i - 1][j + 1] == 'M'
                    && grid[i - 2][j + 2] == 'A'
                    && grid[i - 3][j + 3] == 'S'
                {
                    total += 1;
                    continue;
                }

                // check down-left
                if i < grid.len() - 3
                    && j > 2
                    && grid[i + 1][j - 1] == 'M'
                    && grid[i + 2][j - 2] == 'A'
                    && grid[i + 3][j - 3] == 'S'
                {
                    total += 1;
                    continue;
                }

                // check down-right
                if i < grid.len() - 3
                    && j < grid[i].len() - 3
                    && grid[i + 1][j + 1] == 'M'
                    && grid[i + 2][j + 2] == 'A'
                    && grid[i + 3][j + 3] == 'S'
                {
                    total += 1;
                }
            }
        }

        total
    }

    fn solve_p2(&self, input: &str) -> Self::Part2 {
        let input = input.trim_end().to_string();

        let grid = input
            .split("\n")
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let mut total = 0;

        // look for XMAS in diagonals on the grid
        for i in 1..grid.len() - 1 {
            for j in 1..grid[i].len() - 1 {
                if grid[i][j] == 'A' {
                    let mut d1 = String::from("");
                    d1.push(grid[i - 1][j - 1]);
                    d1.push(grid[i + 1][j + 1]);
                    let mut d2 = String::from("");
                    d2.push(grid[i - 1][j + 1]);
                    d2.push(grid[i + 1][j - 1]);

                    if (d1 == "MS" || d1 == "SM") && (d2 == "MS" || d2 == "SM") {
                        total += 1;
                    }
                }
            }
        }

        total
    }
}
