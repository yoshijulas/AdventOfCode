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

    print!("{checksum}");
}

fn generate_checksum(disk: &[Option<usize>]) -> usize {
    disk.iter()
        .enumerate()
        .filter_map(|(pos, &block)| block.map(|file_id| pos * file_id))
        .sum()
}

fn compact_disk(disk: &mut [Option<usize>]) {
    let mut left = 0;
    let mut right = disk.len();

    while left < right {
        if disk[left].is_none() {
            right -= 1;
            disk.swap(left, right);
        } else {
            left += 1;
        }
    }
}

fn generate_initial_disk_state(line: &[u32]) -> Vec<Option<usize>> {
    let mut very_long_vec = Vec::new();
    let mut current_file_id = 0;

    for (idx, &ch) in line.iter().enumerate() {
        if idx % 2 == 0 {
            very_long_vec.extend(std::iter::repeat(Some(current_file_id)).take(ch as usize));
            current_file_id += 1;
        } else {
            very_long_vec.extend(std::iter::repeat(None).take(ch as usize));
        }
    }
    very_long_vec
}
