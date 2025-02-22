fn main() {
    let grid: Vec<Vec<char>> = std::fs::read_to_string("./day12a/src/input.txt")
        .unwrap()
        .lines()
        .map(|x| x.chars().collect())
        .collect();

    // pretty_print("First", &line);

    let mut total_price = 0;

    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];

    for (idx_col, col) in grid.iter().enumerate() {
        for (idx_row, cell) in col.iter().enumerate() {
            if !visited[idx_col][idx_row] {
                let (area, perimeter) = dfs(idx_col, idx_row, &grid, &mut visited);

                let price = area * perimeter;
                total_price += price;
            }
        }
    }
    print!("{total_price}");
}

fn dfs(x: usize, y: usize, grid: &[Vec<char>], visited: &mut Vec<Vec<bool>>) -> (i32, i32) {
    if x > grid.len() || y > grid[0].len() || visited[x][y] {
        return (0, 0);
    }

    visited[x][y] = true;

    let mut area = 1;
    let mut perimeter = 0;

    let directions: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    for (dx, dy) in directions {
        let new_x = x as isize + dx;
        let new_y = y as isize + dy;

        if new_x < 0
            || new_y < 0
            || new_x >= grid.len() as isize
            || new_y >= grid[0].len() as isize
            || grid[x][y] != grid[new_x as usize][new_y as usize]
            || !visited[x][y]
        {
            perimeter += 1;
        } else {
            let (sub_area, sub_perimeter) = dfs(new_x as usize, new_y as usize, grid, visited);
            area += sub_area;
            perimeter += sub_perimeter;
        }
    }
    (area, perimeter)
}

fn pretty_print<T>(info: &str, arr: &[T])
where
    T: std::fmt::Display,
{
    print!("{info} ");
    for col in arr {
        print!("{col} ");
    }
    println!();
}
