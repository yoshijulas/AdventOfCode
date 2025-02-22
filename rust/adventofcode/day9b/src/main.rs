fn main() {
    let line: Vec<u32> = std::fs::read_to_string("./day9a/src/input.txt")
        .unwrap()
        .chars()
        .filter(|ch| !ch.is_whitespace())
        .map(|x| x.to_digit(10).unwrap())
        .collect();

    let mut disk = generate_initial_disk_state(&line);
    compact_disk(&mut disk);

    let checksum = generate_checksum(&disk);

    print!("checksum: {checksum}");
}

fn generate_checksum(disk: &[Option<usize>]) -> usize {
    disk.iter()
        .enumerate()
        .filter_map(|(pos, &block)| block.map(|file_id| pos * file_id))
        .sum()
}

// Ideas: store all empty spaces len in a vec, then iter and see what fits in it
// check in place size of empty space, then check size of number and see if it fits, if not, keep
// trying

fn compact_disk(disk: &mut [Option<usize>]) {
    let mut files = find_all_numbers(disk);
    files.reverse();

    for (xd, start_idx_files, length) in files {
        if let Some(spaceblock_idx) = find_empty_block(disk, length) {
            if spaceblock_idx < start_idx_files {
                for i in 0..length {
                    disk.swap(spaceblock_idx + i, start_idx_files + i);
                }
            }
        }

        if xd % 5 == 0 {
            pretty_printing("idk", disk);
        }
    }

    let checksum = generate_checksum(disk);

    println!("checksum: {checksum}");
}

fn find_all_empty_space(disk: &[Option<usize>]) -> Vec<(usize, usize)> {
    let mut empty_space = Vec::new();
    let mut i = 0;

    while i < disk.len() {
        if disk[i].is_none() {
            let start = i;
            while i < disk.len() && disk[i].is_none() {
                i += 1;
            }
            empty_space.push((start, i - start));
        } else {
            i += 1;
        }
    }
    empty_space
}

fn find_all_numbers(disk: &[Option<usize>]) -> Vec<(usize, usize, usize)> {
    let mut files = Vec::new();

    let mut i = 0;

    while i < disk.len() {
        if let Some(file_id) = disk[i] {
            let start = i;
            while i < disk.len() && disk[i] == Some(file_id) {
                i += 1;
            }
            files.push((file_id, start, i - start));
        } else {
            i += 1;
        }
    }

    files
}

fn find_empty_block(disk: &[Option<usize>], file: usize) -> Option<usize> {
    disk.windows(file)
        .position(|window| window.iter().all(|&block| block.is_none()))
}

fn generate_initial_disk_state(line: &[u32]) -> Vec<Option<usize>> {
    let mut disk = Vec::new();
    let mut current_file_id = 0;

    for (idx, &ch) in line.iter().enumerate() {
        if idx % 2 == 0 {
            disk.extend(std::iter::repeat(Some(current_file_id)).take(ch as usize));
            current_file_id += 1;
        } else {
            disk.extend(std::iter::repeat(None).take(ch as usize));
        }
    }
    disk
}

fn pretty_printing(info: &str, arr: &[Option<usize>]) {
    println!();
    print!("{info}: ");
    for item in arr {
        if item.is_some() {
            print!("*");
        } else {
            print!(".");
        }
    }
}
