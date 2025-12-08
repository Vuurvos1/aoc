use std::collections::HashMap;

use crate::Solution;

pub struct Day08;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    count: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            count: n,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x == root_y {
            return false;
        }
        if self.rank[root_x] < self.rank[root_y] {
            self.parent[root_x] = root_y;
        } else if self.rank[root_x] > self.rank[root_y] {
            self.parent[root_y] = root_x;
        } else {
            self.parent[root_y] = root_x;
            self.rank[root_x] += 1;
        }
        self.count -= 1;
        true
    }

    fn count_components(&self) -> usize {
        self.count
    }
}

impl Point {
    fn distance_squared(&self, other: &Point) -> i64 {
        let dx = (self.x - other.x) as i64;
        let dy = (self.y - other.y) as i64;
        let dz = (self.z - other.z) as i64;
        dx * dx + dy * dy + dz * dz
    }
}

impl Solution for Day08 {
    type Part1 = usize;
    type Part2 = u64;

    fn solve_p1(&self, input: &str) -> Self::Part1 {
        let junction_boxes: Vec<Point> = input
            .lines()
            .map(|line| {
                let mut parts = line.split(',');
                Point {
                    x: parts.next().unwrap().parse().unwrap(),
                    y: parts.next().unwrap().parse().unwrap(),
                    z: parts.next().unwrap().parse().unwrap(),
                }
            })
            .collect();
        let n = junction_boxes.len();

        let mut pairs: Vec<(usize, usize, i64)> = Vec::new();
        for i in 0..n {
            for j in i + 1..n {
                pairs.push((i, j, junction_boxes[i].distance_squared(&junction_boxes[j])));
            }
        }
        // sort by distance so closest pairs are first
        pairs.sort_unstable_by_key(|&(_, _, dist)| dist);

        let connection_count = 1000;

        let mut uf = UnionFind::new(n);

        for &(i, j, _dist) in pairs.iter().take(connection_count) {
            uf.union(i, j);
        }

        // count circuit sizes
        let mut circuit_sizes: HashMap<usize, usize> = HashMap::new();
        for i in 0..n {
            let root = uf.find(i);
            *circuit_sizes.entry(root).or_insert(0) += 1;
        }

        // Get the three largest circuits
        let mut sizes: Vec<usize> = circuit_sizes.values().copied().collect();
        sizes.sort_unstable_by(|a, b| b.cmp(a));

        sizes[0] * sizes[1] * sizes[2]
    }

    fn solve_p2(&self, input: &str) -> Self::Part2 {
        let junction_boxes: Vec<Point> = input
            .lines()
            .map(|line| {
                let mut parts = line.split(',');
                Point {
                    x: parts.next().unwrap().parse().unwrap(),
                    y: parts.next().unwrap().parse().unwrap(),
                    z: parts.next().unwrap().parse().unwrap(),
                }
            })
            .collect();
        let n = junction_boxes.len();

        let mut pairs: Vec<(usize, usize, i64)> = Vec::new();
        for i in 0..n {
            for j in i + 1..n {
                pairs.push((i, j, junction_boxes[i].distance_squared(&junction_boxes[j])));
            }
        }
        // sort by distance so closest pairs are first
        pairs.sort_unstable_by_key(|&(_, _, dist)| dist);

        let mut uf = UnionFind::new(n);
        let mut last_connection = (0, 0);

        // keep connecting pairs until all boxes are in a circuit
        for &(i, j, _dist) in &pairs {
            if uf.union(i, j) {
                last_connection = (i, j);

                // only 1 component left
                if uf.count_components() == 1 {
                    break;
                }
            }
        }

        let box1 = &junction_boxes[last_connection.0];
        let box2 = &junction_boxes[last_connection.1];

        (box1.x * box2.x) as u64
    }
}
