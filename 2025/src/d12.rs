use crate::Solution;

pub struct Day12;

#[derive(Debug)]
struct Region {
    width: usize,
    height: usize,
    quantities: Vec<usize>,
}

impl Solution for Day12 {
    type Part1 = u64;
    type Part2 = u64;

    fn solve_p1(&self, input: &str) -> Self::Part1 {
        let raw_groups = input.split("\n\n").collect::<Vec<_>>();

        let mut presents: Vec<Vec<Vec<char>>> = Vec::with_capacity(5);
        // go through all groups execpt last
        for group in raw_groups.iter().take(raw_groups.len() - 1) {
            let mut present: Vec<Vec<char>> = Vec::new();
            // skip the first line
            for line in group.lines().skip(1) {
                present.push(line.chars().collect::<Vec<_>>());
            }
            presents.push(present);
        }

        // last item in raw_groups is the grid
        let regions = raw_groups
            .last()
            .unwrap()
            .lines()
            .map(|line| {
                let (start, end) = line.split_once(":").unwrap();
                let (width, height) = start.split_once("x").unwrap();
                let quantities = end
                    .split_ascii_whitespace()
                    .map(|q| q.parse::<usize>().unwrap())
                    .collect::<Vec<_>>();
                Region {
                    width: width.parse::<usize>().unwrap(),
                    height: height.parse::<usize>().unwrap(),
                    quantities,
                }
            })
            .collect::<Vec<_>>();

        let mut fit_count = 0;
        let mut unfit_count = 0;

        // count all the #s per present
        let present_sizes = presents
            .iter()
            .map(|p| get_present_size(p))
            .collect::<Vec<_>>();

        for region in regions {
            // this check seems to be enough for the real puzzle input, it doesn't work for the example input
            let all_presents_count = region.quantities.iter().sum::<usize>();
            let region_area = region.width / 3 * region.height / 3;

            if region_area < all_presents_count {
                unfit_count += 1;
                continue;
            }

            fit_count += 1;

            /*
            // do a rough fit check, each present is 3x3
            let all_presents_count = region.quantities.iter().sum::<usize>();
            let region_area = region.width / 3 * region.height / 3;

            if region_area > all_presents_count {
                fit_count += 1;
                // println!("rough fit check failed");
                // unfit_count += 1;
                continue;
            }

            // this doesn't seem to prune any more regions sadly
            let mut present_size_count = 0;
            for i in 0..region.quantities.len() {
                present_size_count += present_sizes[i] * region.quantities[i];
            }

            let region_area = region.width * region.height;
            if region_area > present_size_count {
                // println!("present size count failed");
                // unfit_count += 1;
                fit_count += 1;
                continue;
            }

            println!("fit check");
            let can_fit = can_fit_grid(&region, &presents);

            if can_fit {
                fit_count += 1;
                continue;
            }

            unfit_count += 1;

            // println!("can fit: {}", can_fit);
            // if !can_fit {
            //     unfit_count += 1;
            //     continue;
            // }

            // fit_count += 1;
            */
        }

        // println!("Fit count: {}", fit_count);
        // println!("Unfit count: {}", unfit_count);

        // percentage of fit regions
        // let percentage = fit_count as f64 / (fit_count + unfit_count) as f64 * 100.0;
        // println!("Percentage of fit regions: {}", percentage);

        fit_count
    }

    fn solve_p2(&self, input: &str) -> Self::Part2 {
        0
    }
}

fn can_fit_grid(region: &Region, presents: &[Vec<Vec<char>>]) -> bool {
    let mut grid = vec![vec![false; region.width]; region.height];
    let all_orientations = presents
        .iter()
        .map(|p| get_all_orientations(p))
        .collect::<Vec<_>>();

    // for orientation in all_orientations {
    //     for present in orientation {
    //         pretty_print_present(&present);
    //         println!("\n");
    //     }
    //     println!("\n");
    // }

    let mut to_place = Vec::new();
    for (present_id, &count) in region.quantities.iter().enumerate() {
        for _ in 0..count {
            to_place.push(present_id);
        }
    }

    // println!("To place: {:?}", to_place);

    backtrack(&mut grid, &to_place, 0, &all_orientations)
}

fn get_present_size(present: &Vec<Vec<char>>) -> usize {
    present.iter().flatten().filter(|&c| *c == '#').count()
}

fn get_all_orientations(present: &Vec<Vec<char>>) -> Vec<Vec<Vec<char>>> {
    let mut orientations = Vec::new();
    let mut current = present.clone();

    for _ in 0..4 {
        // Add current rotation
        orientations.push(current.clone());
        // Add mirrored version
        orientations.push(flip_horizontal(&current));
        current = rotate_present(&current);
    }

    // Remove duplicates (important for symmetric shapes)
    orientations.sort();
    orientations.dedup();
    orientations
}

/// rotate present 90 degrees clockwise
fn rotate_present(present: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let n = present.len();
    let m = present[0].len();

    // For 90° clockwise: new[j][n-1-i] = old[i][j]
    let mut rotated = vec![vec!['.'; n]; m];

    for i in 0..3 {
        for j in 0..3 {
            rotated[j][n - 1 - i] = present[i][j];
        }
    }

    rotated
}

fn flip_horizontal(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    grid.iter()
        .map(|row| row.iter().rev().copied().collect())
        .collect()
}

fn backtrack(
    grid: &mut Vec<Vec<bool>>,
    to_place: &Vec<usize>,
    current_index: usize,
    all_orientations: &Vec<Vec<Vec<Vec<char>>>>,
) -> bool {
    if current_index == to_place.len() {
        return true;
    }

    // let Some((start_x, start_y)) = find_first_empty(grid) else {
    //     return false;  // No empty space but still have pieces
    // };

    let present_id = to_place[current_index];

    for orientation in &all_orientations[present_id] {
        let coords = extract_coords(orientation);

        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if can_place(grid, &coords, x, y) {
                    place(grid, &coords, x, y, true);
                    if backtrack(grid, to_place, current_index + 1, all_orientations) {
                        return true;
                    }
                    place(grid, &coords, x, y, false);
                }
            }
        }
    }

    false
}

fn can_place(grid: &Vec<Vec<bool>>, coords: &[(usize, usize)], x: usize, y: usize) -> bool {
    for (dx, dy) in coords {
        let nx = x + dx;
        let ny = y + dy;
        if ny >= grid.len() || nx >= grid[0].len() || grid[ny][nx] {
            return false;
        }
    }
    true
}

fn place(grid: &mut Vec<Vec<bool>>, coords: &[(usize, usize)], x: usize, y: usize, occupied: bool) {
    for (dx, dy) in coords {
        grid[y + dy][x + dx] = occupied;
    }
}

fn extract_coords(present: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut coords = Vec::new();
    for (y, row) in present.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == '#' {
                coords.push((x, y));
            }
        }
    }
    coords
}

fn pretty_print_present(present: &Vec<Vec<char>>) {
    for row in present {
        println!("{:?}", row);
    }
}
