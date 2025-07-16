use crate::Solution;

pub struct Day04;

impl Solution for Day04 {
    type Part1 = u32;
    type Part2 = u32;

    fn solve_p1(&self, input: &str) -> Self::Part1 {
        let grid = input
            .lines()
            .map(|line| line.bytes().collect::<Vec<_>>())
            .collect::<Vec<Vec<_>>>();

        let mut total = 0;

        let rows = grid.len();
        let cols = grid[0].len();

        // look for XMAS in all directions in grid
        for i in 0..rows {
            for j in 0..cols {
                if grid[i][j] != b'X' {
                    continue;
                }

                // check up
                if i > 2
                    && grid[i - 1][j] == b'M'
                    && grid[i - 2][j] == b'A'
                    && grid[i - 3][j] == b'S'
                {
                    total += 1;
                }

                // check down
                if i < rows - 3
                    && grid[i + 1][j] == b'M'
                    && grid[i + 2][j] == b'A'
                    && grid[i + 3][j] == b'S'
                {
                    total += 1;
                }

                // check left
                if j > 2
                    && grid[i][j - 1] == b'M'
                    && grid[i][j - 2] == b'A'
                    && grid[i][j - 3] == b'S'
                {
                    total += 1;
                }

                // check right
                if j < cols - 3
                    && grid[i][j + 1] == b'M'
                    && grid[i][j + 2] == b'A'
                    && grid[i][j + 3] == b'S'
                {
                    total += 1;
                }

                // check up-left
                if i > 2
                    && j > 2
                    && grid[i - 1][j - 1] == b'M'
                    && grid[i - 2][j - 2] == b'A'
                    && grid[i - 3][j - 3] == b'S'
                {
                    total += 1;
                }

                // check up-right
                if i > 2
                    && j < grid[i].len() - 3
                    && grid[i - 1][j + 1] == b'M'
                    && grid[i - 2][j + 2] == b'A'
                    && grid[i - 3][j + 3] == b'S'
                {
                    total += 1;
                }

                // check down-left
                if i < rows - 3
                    && j > 2
                    && grid[i + 1][j - 1] == b'M'
                    && grid[i + 2][j - 2] == b'A'
                    && grid[i + 3][j - 3] == b'S'
                {
                    total += 1;
                }

                // check down-right
                if i < rows - 3
                    && j < cols - 3
                    && grid[i + 1][j + 1] == b'M'
                    && grid[i + 2][j + 2] == b'A'
                    && grid[i + 3][j + 3] == b'S'
                {
                    total += 1;
                }
            }
        }

        total
    }

    fn solve_p2(&self, input: &str) -> Self::Part2 {
        let grid = input
            .lines()
            .map(|line| line.bytes().collect::<Vec<_>>())
            .collect::<Vec<Vec<_>>>();

        let mut total = 0;

        // look for MAS in diagonals on the grid
        for i in 1..grid.len() - 1 {
            for j in 1..grid[i].len() - 1 {
                if grid[i][j] == b'A' {
                    let top_left = grid[i - 1][j - 1];
                    let bottom_right = grid[i + 1][j + 1];
                    let top_right = grid[i - 1][j + 1];
                    let bottom_left = grid[i + 1][j - 1];

                    if ((top_left == b'M' && bottom_right == b'S')
                        || (top_left == b'S' && bottom_right == b'M'))
                        && ((top_right == b'M' && bottom_left == b'S')
                            || (top_right == b'S' && bottom_left == b'M'))
                    {
                        total += 1;
                    }
                }
            }
        }

        total
    }
}
