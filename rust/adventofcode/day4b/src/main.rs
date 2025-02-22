fn main() {
    let line = std::fs::read_to_string("./day4b/src/input.txt").unwrap();

    let input: Vec<Vec<char>> = line.lines().map(|line| line.chars().collect()).collect();

    let count = xmas_count(&input);

    print!("{count:?}");
}

fn xmas_count(input: &[Vec<char>]) -> i32 {
    // const DIRECTIONS: [[isize; 2]; 4] = [[-1, -1], [1, 1], [1, -1], [-1, 1]];
    const DIRECTIONS: [[isize; 2]; 4] = [[-1, -1], [-1, 1], [1, 1], [1, -1]];
    let mut count = 0;

    for (row_idx, row) in input.iter().enumerate() {
        for (col_idx, &cell) in row.iter().enumerate() {
            if cell == 'A' && is_xmas_pattern_valid(DIRECTIONS, row_idx, col_idx, input, row.len())
            {
                count += 1;
            }
        }
    }
    count
}

fn is_xmas_pattern_valid(
    directions: [[isize; 2]; 4],
    row_idx: usize,
    col_idx: usize,
    input: &[Vec<char>],
    row_len: usize,
) -> bool {
    // let mut letter_count = Letters { m: 0, s: 0 };
    let mut found_letters = String::new();

    for (idx_dir, dir) in directions.iter().enumerate() {
        let new_row = row_idx as isize + dir[0];
        let new_col = col_idx as isize + dir[1];

        if new_row < 0
            || new_col < 0
            || new_row >= input.len() as isize
            || new_col >= row_len as isize
        {
            return false;
        }

        found_letters.push(input[new_row as usize][new_col as usize]);
    }

    if found_letters == "MMSS"
        || found_letters == "SMMS"
        || found_letters == "SSMM"
        || found_letters == "MSSM"
    {
        return true;
    }

    false
    //     let neighbor_char = input[new_row as usize][new_col as usize];
    //
    //     match neighbor_char {
    //         'M' => {
    //             if idx_dir == 0
    //                 || idx_dir == 1 && letter_count.m == 0
    //                 || idx_dir == 2
    //                 || idx_dir == 3 && letter_count.s == 2
    //             {
    //                 letter_count.m += 1;
    //             }
    //         }
    //         'S' => {
    //             if idx_dir == 0
    //                 || idx_dir == 1 && letter_count.s == 0
    //                 || idx_dir == 2
    //                 || idx_dir == 3 && letter_count.m == 2
    //             {
    //                 letter_count.s += 1;
    //             }
    //         }
    //         _ => {
    //             return false;
    //         }
    //     }
    // }
    //
    // if letter_count.m == 2 && letter_count.s == 2 {
    //     return true;
    // }
    // false
}
