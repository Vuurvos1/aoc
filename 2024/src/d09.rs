use crate::Solution;

#[derive(Debug, Clone, Copy)]
struct Space {
    id: u32,
    size: u32,
    file: bool,
}

pub struct Day09;

impl Solution for Day09 {
    type Part1 = i64;
    type Part2 = u64;

    fn solve_p1(&self, input: &str) -> Self::Part1 {
        let mut disk: Vec<i64> = Vec::new();

        let mut file_index = 0;
        let mut digit_count: usize = 0;
        for (i, c) in input.chars().enumerate() {
            let size = c as u32 - 48;
            if i % 2 == 0 {
                // file block
                for _ in 0..size {
                    disk.push(file_index);
                    digit_count += 1;
                }
                file_index += 1;
            } else {
                // padding block
                for _ in 0..size {
                    disk.push(-1);
                }
            }
        }

        let mut skip_count = disk.len();
        for i in 0..digit_count {
            if disk[i] == -1 {
                // get last item from disk that is non empty
                let last_not_dot = disk[0..skip_count].iter().rposition(|x| *x != -1).unwrap();
                skip_count = last_not_dot;
                disk.swap(i, last_not_dot);
            }
        }

        let mut sum: i64 = 0;
        for (i, d) in disk.iter().enumerate() {
            if *d == -1 {
                break;
            }
            sum += i as i64 * *d;
        }

        sum
    }

    fn solve_p2(&self, input: &str) -> Self::Part2 {
        let mut file_disk: Vec<Space> = Vec::new();

        let mut file_index = 0;
        for (i, c) in input.chars().enumerate() {
            let size = c as u32 - 48;
            if i % 2 == 0 {
                // file block
                file_disk.push(Space {
                    id: file_index,
                    size: size,
                    file: true,
                });

                file_index += 1;
                continue;
            }

            // padding block
            file_disk.push(Space {
                id: 0,
                size: size,
                file: false,
            });
        }

        // when inserting into free space, if space left create a new free space at the start of the index
        let mut skip_count = file_disk.len();
        for _ in 0..file_index {
            // get last file from file_disk
            let last_file_index_opt = file_disk[0..skip_count].iter().rposition(|x| x.file);
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
                .position(|x| x.size >= last_file.size && !x.file);

            if let Some(space_index) = space_index {
                // print_disk(&file_disk);
                if space_index > last_file_index {
                    continue;
                }

                let space_left = file_disk[space_index].size - last_file.size;
                file_disk[space_index].size = space_left;

                // move last_file in front of space
                let item = file_disk.remove(last_file_index);
                file_disk.insert(space_index, item);

                // insert empty space from where moved
                file_disk.insert(
                    last_file_index,
                    Space {
                        id: 0,
                        file: false,
                        size: item.size,
                    },
                );
            }
        }

        let mut sum: u64 = 0;
        let mut i: u64 = 0;
        for s in file_disk {
            for _ in 0..s.size {
                if s.file {
                    sum += i * s.id as u64;
                }
                i += 1;
            }
        }

        println!("Hello, world! {}", sum);

        sum
    }
}

fn _print_disk(file_disk: &Vec<Space>) {
    for s in file_disk {
        if s.file {
            for _i in 0..s.size {
                print!("{}", s.id);
            }
            continue;
        }
        // empty space
        if !s.file {
            for _i in 0..s.size {
                print!(".");
            }

            if s.size == 0 {
                print!("()");
            }
        }
    }
    println!()
}
