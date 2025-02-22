fn main() {
    let line: Vec<Vec<u32>> = std::fs::read_to_string("./day10a/src/input.txt")
        .unwrap()
        .lines()
        .map(|x| x.chars().map(|x| x.to_digit(10).unwrap()).collect())
        .collect();

    pretty_print("Before", &line);

    let mut nines: Vec<Vec<Option<usize>>> = line
        .iter()
        .map(|x| {
            x.iter()
                .map(|&x| if x == 9 { Some(0) } else { None })
                .collect()
        })
        .collect();

    for (idx_col, col) in line.iter().enumerate() {
        for (idx_row, &cell) in col.iter().enumerate() {
            if cell == 0 {
                let mut visited = vec![vec![false; line[0].len()]; line.len()];
                dfs(idx_col, idx_row, &line, &mut visited, &mut nines);
            }
        }
    }
    println!();
    pretty_print_option("Nines", &nines);
    let total = nines
        .iter()
        .flatten()
        .fold(0, |acc, &value| value.map_or(acc, |val| acc + val));
    print!("total: {total}");
}

fn dfs(
    x: usize,
    y: usize,
    grid: &[Vec<u32>],
    visited: &mut Vec<Vec<bool>>,
    nines: &mut Vec<Vec<Option<usize>>>,
) {
    if x >= grid.len() || y >= grid[0].len() || visited[x][y] {
        return;
    }

    // Just dont store visited places, so it can search all paths
    // visited[x][y] = true;

    if grid[x][y] == 9 {
        if let Some(count) = &mut nines[x][y] {
            *count += 1;
        }
    }

    let directions: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    for (dx, dy) in directions {
        let new_x = (x as isize + dx) as usize;
        let new_y = (y as isize + dy) as usize;

        if new_x < grid.len() && new_y < grid[0].len() && grid[new_x][new_y] == grid[x][y] + 1 {
            dfs(new_x, new_y, grid, visited, nines);
        }
    }
}

fn pretty_print(info: &str, arr: &[Vec<u32>]) {
    print!("{info} ");
    for col in arr {
        println!();
        for cell in col {
            print!("{cell} ");
        }
    }
    println!();
}
fn pretty_print_option(info: &str, arr: &[Vec<Option<usize>>]) {
    print!("{info} ");
    for col in arr {
        println!();
        for cell in col {
            match cell {
                Some(value) => print!("{value:>2} "),
                None => print!("{:>2} ", "."),
            }
        }
    }
    println!();
}
