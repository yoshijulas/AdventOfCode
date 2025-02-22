use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use crossterm::style::Stylize;

#[derive(Hash, PartialEq, Eq, Ord, PartialOrd, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

type Node = (Point, u8);

fn main() {
    let input = std::fs::read_to_string("./day16a/src/input.txt").unwrap();

    let grid: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();

    let mut start = (Point { x: 0, y: 0 }, 0);
    let mut end = Point { x: 0, y: 0 };

    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, &cell) in row.iter().enumerate() {
            if cell == 'E' {
                end = Point {
                    x: row_idx as isize,
                    y: col_idx as isize,
                };
            } else if cell == 'S' {
                start = (
                    Point {
                        x: row_idx as isize,
                        y: col_idx as isize,
                    },
                    0,
                );
            }
        }
    }

    match dijkstra(&grid, start, end) {
        Some((distance, path)) => {
            println!("Shortest distance: {distance}");
            println!("Steps: {}", path.len() - 2);
            for (row_idx, row) in grid.iter().enumerate() {
                for (col_idx, cell) in row.iter().enumerate() {
                    let mut printed = false;
                    for poin in &path {
                        if poin.0.x == row_idx as isize && poin.0.y == col_idx as isize {
                            match poin.1 {
                                0 => print!("{}", ">".red()),
                                1 => print!("{}", "v".red()),
                                2 => print!("{}", "<".red()),
                                3 => print!("{}", "^".red()),
                                _ => {}
                            }
                            printed = true;
                            break;
                        }
                    }
                    if !printed {
                        print!("{cell}");
                    }
                }
                println!();
            }
        }
        None => println!("No path found"),
    }
}

fn dijkstra(grid: &[Vec<char>], start: Node, end: Point) -> Option<(i32, Vec<Node>)> {
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    let rows = grid.len();
    let cols = grid[0].len();

    let mut distances = HashMap::new();
    let mut heap = BinaryHeap::new();
    let mut predecessors = HashMap::new();

    distances.insert(start, 0);
    heap.push(Reverse((0, start)));

    while let Some(Reverse((dist, node))) = heap.pop() {
        if node.0 == end {
            println!("{dist}");
            let mut path = vec![(end, node.1)];
            let mut current = (end, node.1);
            while let Some(&prev) = predecessors.get(&current) {
                path.push(prev);
                current = prev;
            }
            path.reverse();
            return Some((dist, path));
        }

        for (new, weight) in adj(grid, node, &directions) {
            if new.0.x < 0 || new.0.y < 0 || new.0.x >= rows as isize || new.0.y >= cols as isize {
                continue;
            }

            let dist_w = *distances.get(&new).unwrap_or(&i32::MAX);
            let new_dist = dist + weight;
            if new_dist < dist_w {
                distances.insert(new, new_dist);
                predecessors.insert(new, node);
                heap.push(Reverse((new_dist, new)));
            }
        }
    }
    None
}

fn adj(grid: &[Vec<char>], (curr, dir): Node, directions: &[(isize, isize)]) -> Vec<(Node, i32)> {
    let mut adj = Vec::new();
    let (nx, ny) = (
        curr.x + directions[dir as usize].0,
        curr.y + directions[dir as usize].1,
    );
    let neighbor = Point { x: nx, y: ny };

    if grid[neighbor.x as usize][neighbor.y as usize] != '#' {
        adj.push(((neighbor, dir), 1));
    }
    adj.push(((curr, (dir + 3) % 4), 1000));
    adj.push(((curr, (dir + 1) % 4), 1000));
    adj
}
