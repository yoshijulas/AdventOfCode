fn main() {
    let line = std::fs::read_to_string("./day4a/src/input.txt").unwrap();

    let input: Vec<Vec<_>> = line.lines().map(|line| line.chars().collect()).collect();

    let posibilities = [
        [0, 1],
        [0, -1],
        [1, 0],
        [-1, 0],
        [1, 1],
        [-1, -1],
        [1, -1],
        [-1, 1],
    ];
    let word = "XMAS";
    let mut count = 0;

    for (row_idx, row) in input.iter().enumerate() {
        for (col_idx, &cell) in row.iter().enumerate() {
            if cell == 'X' {
                for dir in posibilities {
                    let mut found = true;

                    for (i, char) in word.chars().enumerate() {
                        let new_row = row_idx as isize + i as isize * dir[0];
                        let new_col = col_idx as isize + i as isize * dir[1];

                        if new_row < 0
                            || new_col < 0
                            || new_row >= input.len() as isize
                            || new_col >= row.len() as isize
                            || input[new_row as usize][new_col as usize] != char
                        {
                            found = false;
                            break;
                        }
                    }
                    if found {
                        count += 1;
                    }
                }
            }
        }
    }

    print!("{count:?}");
}
