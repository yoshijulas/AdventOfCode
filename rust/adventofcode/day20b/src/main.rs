use std::collections::{HashSet, VecDeque};

use crossterm::style::Stylize;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

fn main() {
    let input = std::fs::read_to_string("./day20a/src/input.txt").unwrap();

    let grid: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();

    let start = find_values(&grid, 'S');
    let mut dists = vec![vec![-1; grid[0].len()]; grid.len()];

    bfs(&grid, start, &mut dists);

    let threshold = 100;
    let mut count = 0;

    for row_idx in 0..dists[0].len() {
        for col_idx in 0..dists.len() {
            if grid[row_idx][col_idx] == '#' {
                continue;
            }
            for radius in 2..21 {
                for nx in 0..=radius {
                    let ny = radius - nx;
                    let (row_idx, col_idx) = (row_idx as isize, col_idx as isize);
                    for (dx, dy) in HashSet::from([
                        (row_idx + nx, col_idx + ny),
                        (row_idx + nx, col_idx - ny),
                        (row_idx - nx, col_idx + ny),
                        (row_idx - nx, col_idx - ny),
                    ]) {
                        if dx < 0
                            || dy < 0
                            || dx >= grid.len() as isize
                            || dy >= grid[0].len() as isize
                        {
                            continue;
                        }
                        let (dx, dy) = (dx as usize, dy as usize);
                        if grid[dx][dy] == '#' {
                            continue;
                        }

                        if (dists[row_idx as usize][col_idx as usize] - dists[dx][dy]) as isize
                            >= threshold + radius
                        {
                            count += 1;
                        }
                    }
                }
            }
        }
    }

    println!("{count}");

    // for row in dists {
    //     for cell in row {
    //         print!("{cell:>5}");
    //     }
    //     println!();
    // }

    // print!("{posibles}");
}

fn find_values(grid: &[Vec<char>], ch_to_find: char) -> Point {
    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, &cell) in row.iter().enumerate() {
            if cell == ch_to_find {
                return Point {
                    x: row_idx,
                    y: col_idx,
                };
            }
        }
    }
    Point { x: 0, y: 0 }
}

fn bfs(grid: &[Vec<char>], start: Point, dists: &mut [Vec<i32>]) {
    dists[start.x][start.y] = 0;
    let mut queue = VecDeque::new();

    queue.push_back(start);
    while let Some(curr) = queue.pop_front() {
        for (dx, dy) in &DIRECTIONS {
            let (nx, ny) = (curr.x as isize + dx, curr.y as isize + dy);
            if nx < 0 || ny < 0 || nx >= grid.len() as isize || ny >= grid[0].len() as isize {
                continue;
            }
            let (nx, ny) = (nx as usize, ny as usize);

            if grid[nx][ny] == '#' {
                continue;
            }
            if dists[nx][ny] != -1 {
                continue;
            }
            dists[nx][ny] = dists[curr.x][curr.y] + 1;
            queue.push_back(Point { x: nx, y: ny });
        }
    }
}
