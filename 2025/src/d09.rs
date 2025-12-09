use crate::Solution;

pub struct Day09;

impl Solution for Day09 {
    type Part1 = u64;
    type Part2 = u64;

    fn solve_p1(&self, input: &str) -> Self::Part1 {
        let corners = input
            .lines()
            .map(|line| {
                let mut parts = line.split(',');
                (
                    parts.next().unwrap().parse().unwrap(),
                    parts.next().unwrap().parse().unwrap(),
                )
            })
            .collect::<Vec<(u64, u64)>>();

        let mut max_area = 0;

        for i in 0..corners.len() {
            for j in i + 1..corners.len() {
                let area = (corners[i].0.abs_diff(corners[j].0) + 1)
                    * (corners[i].1.abs_diff(corners[j].1) + 1);

                if area > max_area {
                    max_area = area;
                }
            }
        }

        max_area
    }

    fn solve_p2(&self, input: &str) -> Self::Part2 {
        let corners = input
            .lines()
            .map(|line| {
                let mut parts = line.split(',');
                (
                    parts.next().unwrap().parse().unwrap(),
                    parts.next().unwrap().parse().unwrap(),
                )
            })
            .collect::<Vec<(i64, i64)>>();

        let mut max_area = 0;

        for i in 0..corners.len() {
            for j in i + 1..corners.len() {
                let (x1, y1) = corners[i];
                let (x2, y2) = corners[j];

                let min_x = x1.min(x2);
                let max_x = x1.max(x2);
                let min_y = y1.min(y2);
                let max_y = y1.max(y2);

                let area = (max_x - min_x + 1) * (max_y - min_y + 1);
                if area <= max_area {
                    continue;
                }

                // check if this rectangle is entirely inside the polygon
                if is_rectangle_inside_polygon(&corners, min_x, max_x, min_y, max_y) {
                    let area = (max_x - min_x + 1) * (max_y - min_y + 1);
                    max_area = max_area.max(area);
                }
            }
        }

        max_area as u64
    }
}

fn is_rectangle_inside_polygon(
    polygon: &Vec<(i64, i64)>,
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
) -> bool {
    // check all 4 corners of rectangle are inside
    let rect_corners = [
        (min_x, min_y),
        (min_x, max_y),
        (max_x, min_y),
        (max_x, max_y),
    ];

    for &(x, y) in &rect_corners {
        if !point_inside_polygon(polygon, x, y) {
            return false;
        }
    }

    // check if no polygon edges cross through the rectangle
    let n = polygon.len();
    for i in 0..n {
        let (x1, y1) = polygon[i];
        let (x2, y2) = polygon[(i + 1) % n];

        // horizontal edge
        if y1 == y2 {
            let edge_y = y1;
            let edge_min_x = x1.min(x2);
            let edge_max_x = x1.max(x2);

            // check if horizontal edge crosses through the rectangle interior
            if edge_y > min_y && edge_y < max_y {
                if edge_min_x < max_x && edge_max_x > min_x {
                    return false;
                }
            }
        }

        // vertical edge
        if x1 == x2 {
            let edge_x = x1;
            let edge_min_y = y1.min(y2);
            let edge_max_y = y1.max(y2);

            // check if vertical edge crosses through the rectangle interior
            if edge_x > min_x && edge_x < max_x {
                if edge_min_y < max_y && edge_max_y > min_y {
                    return false;
                }
            }
        }
    }

    true
}

fn point_inside_polygon(polygon: &Vec<(i64, i64)>, x: i64, y: i64) -> bool {
    let mut inside = false;
    let n = polygon.len();

    // check if point is on the polygon edge
    if polygon.iter().any(|&(px, py)| px == x && py == y) {
        return true;
    }

    for i in 0..n {
        let (x1, y1) = (polygon[i].0 as i64, polygon[i].1 as i64);
        let (x2, y2) = (polygon[(i + 1) % n].0 as i64, polygon[(i + 1) % n].1 as i64);

        // ray casting algorithm
        if (y1 > y) != (y2 > y) {
            let slope_x = x1 + (x2 - x1) * (y - y1) / (y2 - y1);
            if x < slope_x {
                inside = !inside;
            }
        }
    }

    inside
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    fn get_test_corners() -> Vec<(i64, i64)> {
        TEST_INPUT
            .lines()
            .map(|line| {
                let mut parts = line.split(',');
                (
                    parts.next().unwrap().parse().unwrap(),
                    parts.next().unwrap().parse().unwrap(),
                )
            })
            .collect()
    }

    #[test]
    fn test_example_max_area() {
        let corners = get_test_corners();
        // 9,5 and 2,3
        let result = is_rectangle_inside_polygon(&corners, 2, 9, 3, 5);
        assert_eq!(result, true);
    }
}
