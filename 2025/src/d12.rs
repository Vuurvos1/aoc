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

        // 4x4: 0 0 0 0 2 0
        let mut presents: Vec<Vec<char>> = Vec::with_capacity(5);
        // go through all groups execpt last
        for group in raw_groups.iter().take(raw_groups.len() - 1) {
            // skip the first line
            for line in group.lines().skip(1) {
                presents.push(line.chars().collect::<Vec<_>>());
            }
        }

        println!("{:?}", presents);

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

        println!("{:?}", regions);

        let mut fit_count = 0;
        let mut unfit_count = 0;

        for region in regions {
            // do a rough fit check, each present is 3x3
            let all_presents_count = region.quantities.iter().sum::<usize>();
            let region_area = region.width / 3 * region.height / 3;

            if region_area < all_presents_count {
                println!(
                    "Region {} is too small for presents",
                    region.width * region.height
                );
                unfit_count += 1;
                continue;
            }

            println!(
                "Region {} is big enough for presents",
                region.width * region.height
            );
            fit_count += 1;

            // TODO: check if there are more #s than the area of the region
        }

        println!("Fit count: {}", fit_count);
        println!("Unfit count: {}", unfit_count);

        // percentage of fit regions
        let percentage = fit_count as f64 / (fit_count + unfit_count) as f64 * 100.0;
        println!("Percentage of fit regions: {}", percentage);

        0
    }

    fn solve_p2(&self, input: &str) -> Self::Part2 {
        0
    }
}
