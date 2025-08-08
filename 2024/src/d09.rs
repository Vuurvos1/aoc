use crate::Solution;

#[derive(Debug, Clone, Copy)]
struct PackedSpace(u64);

impl PackedSpace {
    fn new(id: u64, size: u8, file: bool) -> Self {
        // Bits 0-3: size
        // Bits 4-62: file_id
        // Bit 63: is_file flag
        PackedSpace((size as u64) | (id << 4) | ((file as u64) << 63))
    }

    fn size(&self) -> u64 {
        self.0 & 0xF
    }

    fn id(&self) -> u64 {
        (self.0 & 0x7FFFFFFFFFFFFFFF) >> 4 // Clear file bit, then shift
    }

    fn is_file(&self) -> bool {
        (self.0 & 0x8000000000000000) != 0 // Check bit 63 explicitly
    }
}

pub struct Day09;

impl Solution for Day09 {
    type Part1 = i64;
    type Part2 = u64;

    fn solve_p1(&self, input: &str) -> Self::Part1 {
        let total_size: usize = input.as_bytes().iter().map(|&c| (c - b'0') as usize).sum();
        let mut disk = Vec::with_capacity(total_size);

        let mut file_index = 0;
        let mut digit_count: usize = 0;
        for (i, &c) in input.as_bytes().iter().enumerate() {
            let size = (c - b'0') as usize;
            if i % 2 == 0 {
                // file block
                let start_len = disk.len();
                disk.resize(start_len + size, file_index);
                digit_count += size;
                file_index += 1;
            } else {
                // padding block
                let start_len = disk.len();
                disk.resize(start_len + size, -1);
            }
        }

        // merge disk end items in array info first empty space
        // 0..1..22
        // 02.1..2.
        // 0221....

        let mut left = 0;
        let mut right = disk.len() - 1;

        while left < right {
            // find first empty space
            while left < right && disk[left] != -1 {
                left += 1;
            }

            // find last empty
            while left < right && disk[right] == -1 {
                right -= 1;
            }

            // Swap if we found both
            if left < right {
                disk.swap(left, right);
                left += 1;
                right -= 1;
            }
        }

        disk.iter()
            .take(digit_count)
            .enumerate()
            .map(|(i, d)| i as i64 * *d)
            .sum()
    }

    fn solve_p2(&self, input: &str) -> Self::Part2 {
        // TODO: maybe rework in a way where each disk item has a position and size and we update these instead of moving them

        let estimated_segments = input.len();
        let mut file_disk: Vec<PackedSpace> = Vec::with_capacity(estimated_segments);

        let mut file_index = 0;
        for (i, c) in input.as_bytes().iter().enumerate() {
            let size = c - b'0';
            if i % 2 == 0 {
                // file block
                file_disk.push(PackedSpace::new(file_index, size, true));

                file_index += 1;
                continue;
            }

            // padding block
            file_disk.push(PackedSpace::new(0, size, false));
        }

        // when inserting into free space, if space left create a new free space at the start of the index
        let mut skip_count = file_disk.len();
        for _ in 0..file_index {
            // get last file from file_disk
            let last_file_index_opt = file_disk[0..skip_count].iter().rposition(|x| x.is_file());
            if last_file_index_opt.is_none() {
                skip_count -= 1;
                continue;
            }
            let last_file_index = last_file_index_opt.unwrap();
            let last_file = file_disk[last_file_index];
            skip_count = last_file_index;

            // get first free space that fits
            let space_index = file_disk
                .iter()
                .position(|x| x.size() >= last_file.size() && !x.is_file());

            if let Some(space_index) = space_index {
                // print_disk(&file_disk);
                if space_index > last_file_index {
                    continue;
                }

                let space_left = file_disk[space_index].size() - last_file.size();
                file_disk[space_index] = PackedSpace::new(0, space_left as u8, false);

                // move last_file in front of space
                let item = file_disk.remove(last_file_index);
                file_disk.insert(space_index, item);

                // insert empty space from where moved
                file_disk.insert(
                    last_file_index,
                    PackedSpace::new(0, item.size() as u8, false),
                );
            }
        }

        let mut sum: u64 = 0;
        let mut position: u64 = 0;
        for s in file_disk {
            let size = s.size() as u64;
            if s.is_file() {
                sum += s.id() * (size * position + size * (size - 1) / 2);
            }
            position += size;
        }

        sum
    }
}

fn _print_packed_disk(file_disk: &Vec<PackedSpace>) {
    for s in file_disk {
        if s.is_file() {
            print!("({},{})", s.id(), s.size());
            continue;
        }

        for _ in 0..s.size() {
            print!(".");
        }
    }

    println!()
}

#[test]
fn test_packed_space_basic() {
    // Test file space
    let file_space = PackedSpace::new(42, 5, true);
    assert_eq!(file_space.id(), 42);
    assert_eq!(file_space.size(), 5);
    assert_eq!(file_space.is_file(), true);

    // Test free space
    let free_space = PackedSpace::new(0, 3, false);
    assert_eq!(free_space.id(), 0);
    assert_eq!(free_space.size(), 3);
    assert_eq!(free_space.is_file(), false);
}

#[test]
fn part2_example() {
    assert_eq!(Day09.solve_p2("2333133121414131402"), 2858);
}
