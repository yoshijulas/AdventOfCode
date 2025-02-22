use std::collections::{HashSet, VecDeque};

use crossterm::style::Stylize;

#[derive(Debug, PartialEq, PartialOrd, Hash, Eq, Clone, Copy)]
struct Coords {
    x: usize,
    y: usize,
}

fn main() {
    let input = std::fs::read_to_string("./day18a/src/input.txt").unwrap();

    let bytes: Vec<Coords> = input
        .lines()
        .map(|x| {
            let coord: Vec<usize> = x.split(',').map(|x| x.parse().unwrap()).collect();
            Coords {
                x: coord[1],
                y: coord[0],
            }
        })
        .collect();
    let len = 71;
    let take = 1024;

    let mut grid = vec![vec!['.'; len]; len];

    for (byte_idx, byte) in bytes.iter().take(take).enumerate() {
        grid[byte.x][byte.y] = '#';
    }
    // let mut grid_set = HashSet::new();

    // for byte in &bytes[..15] {
    //     grid_set.insert(byte);
    // }

    let start = Coords { x: 0, y: 0 };
    let end = Coords {
        x: len - 1,
        y: len - 1,
    };

    let steps = bfs(&grid, start, end, len);
    println!("steps: {steps}");

    pretty_print(&grid);
}

fn pretty_print(grid: &[Vec<char>]) {
    for row in grid {
        for cell in row {
            print!("{cell}");
        }
        println!();
    }
}

fn bfs(grid: &[Vec<char>], start: Coords, end: Coords, grid_size: usize) -> usize {
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)]; // Right, Down, Left, Up

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back((start, 0)); // (current_position, steps)
    visited.insert(start);

    while let Some((curr, steps)) = queue.pop_front() {
        if curr == end {
            return steps;
        }

        for &(dx, dy) in &directions {
            let nx = curr.x as isize + dx;
            let ny = curr.y as isize + dy;

            if nx >= 0 && ny >= 0 && nx < grid_size as isize && ny < grid_size as isize {
                let next = Coords {
                    x: nx as usize,
                    y: ny as usize,
                };
                if grid[next.x][next.y] != '#' && !visited.contains(&next) {
                    visited.insert(next);
                    queue.push_back((next, steps + 1));
                }
            }
        }
    }
    0
}
