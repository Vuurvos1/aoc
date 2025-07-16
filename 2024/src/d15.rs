use std::collections::VecDeque;
// use std::{thread, time::Duration};

use crate::Solution;

pub struct Day15;

impl Solution for Day15 {
    type Part1 = usize;
    type Part2 = usize;

    fn solve_p1(&self, input: &str) -> Self::Part1 {
        let input: Vec<&str> = input.split("\n\n").collect();

        let mut grid: Vec<Vec<char>> = input[0]
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        let moves: Vec<char> = input[1].chars().collect();

        // (y, x)
        let mut pos = (0, 0);
        'outer: for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if grid[y][x] == '@' {
                    pos = (y as i32, x as i32);
                    break 'outer;
                }
            }
        }

        for m in moves {
            // println!("Move: {}", m);
            match m {
                '^' => pos = move_grid(pos, (-1, 0), &mut grid),
                '>' => pos = move_grid(pos, (0, 1), &mut grid),
                '<' => pos = move_grid(pos, (0, -1), &mut grid),
                'v' => pos = move_grid(pos, (1, 0), &mut grid),
                _ => (), // println!("Invalid move {}", m),
            }

            // print_grid(&grid);
            // println!("");
        }

        // score
        let mut sum = 0;
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if grid[y][x] == 'O' {
                    sum += y * 100 + x;
                }
            }
        }

        sum
    }

    fn solve_p2(&self, input: &str) -> Self::Part2 {
        let input: Vec<&str> = input.split("\n\n").collect();

        let grid: Vec<Vec<char>> = input[0]
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        let moves: Vec<char> = input[1].chars().collect();

        // double width, same height
        let mut wide_grid: Vec<Vec<char>> = vec![vec![]; grid.len()];
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                match grid[y][x] {
                    '@' => wide_grid[y].extend(['@', '.']),
                    'O' => wide_grid[y].extend(['[', ']']),
                    '#' => wide_grid[y].extend(['#', '#']),
                    _ => wide_grid[y].extend(['.', '.']),
                }
            }
        }

        // (y, x)
        let mut pos = (0, 0);
        'outer: for y in 0..wide_grid.len() {
            for x in 0..wide_grid[y].len() {
                if wide_grid[y][x] == '@' {
                    pos = (y as i32, x as i32);
                    wide_grid[y][x] = '.';
                    break 'outer;
                }
            }
        }

        let mut boxes_to_move: Vec<(i32, i32)> = Vec::new();
        let mut positions_to_check: VecDeque<(i32, i32)> = VecDeque::new();

        // print!("{}", "\x1B[2J\x1B[1;1H");

        for m in moves {
            let direction = match m {
                '^' => (-1, 0),
                '>' => (0, 1),
                '<' => (0, -1),
                'v' => (1, 0),
                _ => (0, 0),
            };
            let mut blocked = false;
            positions_to_check.push_back((pos.0 + direction.0, pos.1 + direction.1));

            while let Some((y, x)) = positions_to_check.pop_front() {
                if !inbounds(x, y, wide_grid[0].len(), wide_grid.len()) {
                    continue;
                }

                if let Some(box_to_move) = match wide_grid[y as usize][x as usize] {
                    '.' => None,
                    '#' => {
                        blocked = true;
                        break;
                    }
                    '[' => Some((y, x)),
                    ']' => Some((y, x - 1)),
                    _ => None,
                } {
                    if !boxes_to_move.contains(&box_to_move) {
                        boxes_to_move.push(box_to_move);
                        if direction != (0, 1) {
                            positions_to_check.push_back((
                                box_to_move.0 + direction.0,
                                box_to_move.1 + direction.1,
                            ));
                        }
                        if direction != (0, -1) {
                            positions_to_check.push_back((
                                box_to_move.0 + direction.0,
                                box_to_move.1 + 1 + direction.1,
                            ));
                        }
                    }
                }
            }

            if !blocked {
                pos.0 += direction.0;
                pos.1 += direction.1;
                for &(box_row, box_col) in boxes_to_move.iter().rev() {
                    wide_grid[box_row as usize][box_col as usize] = '.';
                    wide_grid[box_row as usize][(box_col + 1) as usize] = '.';
                    wide_grid[(box_row + direction.0) as usize][(box_col + direction.1) as usize] =
                        '[';
                    wide_grid[(box_row + direction.0) as usize]
                        [(box_col + 1 + direction.1) as usize] = ']';
                }
            }

            boxes_to_move.clear();
            positions_to_check.clear();

            // print_grid(&wide_grid, pos);
            // println!("");
            // thread::sleep(Duration::from_millis(1));
        }

        let mut sum = 0;
        for y in 0..wide_grid.len() {
            for x in 0..wide_grid[y].len() {
                if wide_grid[y][x] == '[' {
                    sum += y * 100 + x;
                }
            }
        }

        sum
    }
}

fn inbounds(x: i32, y: i32, width: usize, height: usize) -> bool {
    x >= 0 && x < width as i32 && y >= 0 && y < height as i32
}

// move the robot around the grid and push "O" boxes, "#" walls
fn move_grid(pos: (i32, i32), dir: (i32, i32), grid: &mut Vec<Vec<char>>) -> (i32, i32) {
    let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
    if !inbounds(new_pos.1, new_pos.0, grid.len(), grid[0].len()) {
        return new_pos;
    }

    // wall
    if grid[new_pos.0 as usize][new_pos.1 as usize] == '#' {
        return pos;
    }

    // empty space
    if grid[new_pos.0 as usize][new_pos.1 as usize] == '.' {
        grid[new_pos.0 as usize][new_pos.1 as usize] = '@';
        grid[pos.0 as usize][pos.1 as usize] = '.';
        return new_pos;
    }

    if grid[new_pos.0 as usize][new_pos.1 as usize] == 'O' {
        // look for free space
        let mut box_positions = vec![];
        let mut box_pos = new_pos;
        let mut can_push = true;
        loop {
            box_positions.push(box_pos);
            box_pos = (box_pos.0 + dir.0, box_pos.1 + dir.1);
            if !inbounds(box_pos.1, box_pos.0, grid.len(), grid[0].len()) {
                break;
            }

            if grid[box_pos.0 as usize][box_pos.1 as usize] == '#' {
                can_push = false;
                break;
            }

            if grid[box_pos.0 as usize][box_pos.1 as usize] == 'O' {
                continue;
            }

            if grid[box_pos.0 as usize][box_pos.1 as usize] == '.' {
                break;
            }
        }

        if can_push {
            for box_pos in box_positions.iter() {
                grid[(box_pos.0 + dir.0) as usize][(box_pos.1 + dir.1) as usize] = 'O';
            }
            grid[new_pos.0 as usize][new_pos.1 as usize] = '@';
            grid[pos.0 as usize][pos.1 as usize] = '.';
            return new_pos;
        }
    }

    (new_pos.0 - dir.0, new_pos.1 - dir.1)
}

fn _print_grid(grid: &Vec<Vec<char>>, pos: (i32, i32)) {
    let mut frame = String::new();
    frame.push_str("\x1B[H");

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if x == pos.1 as usize && y == pos.0 as usize {
                frame.push_str("\x1B[1;36m@\x1B[0m");
                continue;
            }
            frame.push_str(match grid[y][x] {
                'O' => "\x1B[1;32mO\x1B[0m",
                '#' => "\x1B[1;31m#\x1B[0m",
                '[' => "\x1B[1;33m[\x1B[0m",
                ']' => "\x1B[1;33m]\x1B[0m",
                _ => ".",
            });
        }
        frame.push('\n');
    }
    print!("{}", frame);
}
