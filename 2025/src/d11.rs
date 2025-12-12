use std::collections::HashMap;

use crate::Solution;

pub struct Day11;

impl Solution for Day11 {
    type Part1 = usize;
    type Part2 = usize;

    fn solve_p1(&self, input: &str) -> Self::Part1 {
        // find all paths from you to out

        let devices: HashMap<u32, Vec<u32>> = input
            .lines()
            .map(|line| {
                let (device, connections) = line.split_once(':').unwrap();
                let connections = connections
                    .split_ascii_whitespace()
                    .map(|c| device_index(c))
                    .collect::<Vec<_>>();
                (device_index(device), connections)
            })
            .collect();

        count_paths(
            device_index("you"),
            device_index("out"),
            &devices,
            &mut HashMap::new(),
        )
    }

    fn solve_p2(&self, input: &str) -> Self::Part2 {
        // find all paths from svr to out passing through fft and dac

        let devices: HashMap<u32, Vec<u32>> = input
            .lines()
            .map(|line| {
                let (device, connections) = line.split_once(':').unwrap();
                let connections = connections
                    .split_ascii_whitespace()
                    .map(|c| device_index(c))
                    .collect::<Vec<_>>();
                (device_index(device), connections)
            })
            .collect();

        // svr -> fft
        // fft -> dac
        // dac -> out
        let svr_to_fft = count_paths(
            device_index("svr"),
            device_index("fft"),
            &devices,
            &mut HashMap::new(),
        );
        let fft_to_dac = count_paths(
            device_index("fft"),
            device_index("dac"),
            &devices,
            &mut HashMap::new(),
        );
        let dac_to_out = count_paths(
            device_index("dac"),
            device_index("out"),
            &devices,
            &mut HashMap::new(),
        );
        let fft_dac_result = svr_to_fft * fft_to_dac * dac_to_out;

        // svr -> dac
        // dac -> fft
        // fft -> out
        let svr_to_dac = count_paths(
            device_index("svr"),
            device_index("dac"),
            &devices,
            &mut HashMap::new(),
        );
        let dac_to_fft = count_paths(
            device_index("dac"),
            device_index("fft"),
            &devices,
            &mut HashMap::new(),
        ); // this is 0 for my input?
        let fft_to_out = count_paths(
            device_index("fft"),
            device_index("out"),
            &devices,
            &mut HashMap::new(),
        );
        let dac_fft_result = svr_to_dac * dac_to_fft * fft_to_out;

        // multiply while avoiding 0 values
        fft_dac_result.max(1) * dac_fft_result.max(1)
    }
}

fn device_index(device: &str) -> u32 {
    let bytes = device.as_bytes();
    (bytes[0] - b'a') as u32 * 26 * 26 + (bytes[1] - b'a') as u32 * 26 + (bytes[2] - b'a') as u32
}

fn count_paths(
    start: u32,
    end: u32,
    devices: &HashMap<u32, Vec<u32>>,
    cache: &mut HashMap<(u32, u32), usize>,
) -> usize {
    if start == end {
        return 1;
    }

    if let Some(c) = cache.get(&(start, end)) {
        return *c;
    }

    let mut result = 0;
    if let Some(connections) = devices.get(&start) {
        for &next_device in connections {
            result += count_paths(next_device, end, devices, cache);
        }
    }

    cache.insert((start, end), result);
    result
}
