struct Point {
    x: usize,
    y: usize,
}

struct Dir {
    x: i32,
    y: i32,
}

fn main() {
    let input = std::fs::read_to_string("./day15a/src/input.txt").unwrap();

    let input_split: Vec<&str> = input.split("\r\n\r\n").map(|x| x.trim()).collect();

    let mut grid: Vec<Vec<char>> = input_split[0]
        .lines()
        .map(|x| x.chars().collect())
        .collect();
    let moves = input_split[1].replace("\r\n", "");

    let mut robot = find_robot(&grid);

    for col in &grid {
        println!();
        for cell in col {
            print!("{cell}");
        }
    }
    println!();

    for m in moves.chars() {
        move_position(&mut grid, m, &mut robot);
    }

    for col in &grid {
        println!();
        for cell in col {
            print!("{cell}");
        }
    }
    println!();

    let mut sum = 0;
    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, &cell) in row.iter().enumerate() {
            if cell == 'O' {
                sum += 100 * row_idx + col_idx;
            }
        }
    }

    println!("{sum}");
}

fn find_robot(grid: &[Vec<char>]) -> Point {
    for (row_idx, row) in grid.iter().enumerate() {
        if let Some(col_idx) = row.iter().position(|&c| c == '@') {
            return Point {
                x: col_idx,
                y: row_idx,
            };
        }
    }
    unreachable!("It should have a robot");
}

fn move_position(grid: &mut [Vec<char>], dir: char, position: &mut Point) {
    let dirs = match dir {
        '<' => Dir { x: 0, y: -1 },
        '>' => Dir { x: 0, y: 1 },
        '^' => Dir { x: -1, y: 0 },
        'v' => Dir { x: 1, y: 0 },
        _ => Dir { x: 0, y: 0 },
    };

    if dirs.x == 0 && dirs.y == 0 {
        eprint!("there must be a value here");
    }

    let mut blocked = true;
    let mut steps = 1;
    let mut empty_space = Point { x: 0, y: 0 };

    while blocked {
        let new_x = (position.x as i32 + dirs.x * steps) as usize;
        let new_y = (position.y as i32 + dirs.y * steps) as usize;
        let new_position = grid[new_x][new_y];

        if new_position == '#' {
            return;
        } else if new_position == '.' {
            empty_space = Point { x: new_x, y: new_y };
            blocked = false;
        } else if new_position == 'O' {
            steps += 1;
        } else {
            unimplemented!("There are no more positions");
        }
    }
    let next_x = (position.x as i32 + dirs.x) as usize;
    let next_y = (position.y as i32 + dirs.y) as usize;

    if steps != 1 {
        let temp = grid[empty_space.x][empty_space.y];
        grid[empty_space.x][empty_space.y] = grid[next_x][next_y];
        grid[next_x][next_y] = temp;
    }

    let temp = grid[next_x][next_y];
    grid[next_x][next_y] = grid[position.x][position.y];
    grid[position.x][position.y] = temp;

    *position = Point {
        x: next_x,
        y: next_y,
    };
}
