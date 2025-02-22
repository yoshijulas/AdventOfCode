use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use crossterm::style::Stylize;

#[derive(Hash, PartialEq, Eq, Ord, PartialOrd, Clone, Copy, Debug, Default)]
struct Point {
    x: isize,
    y: isize,
}

type Node = (Point, u8);

fn main() {
    let input = std::fs::read_to_string("./day16b/src/input.txt").unwrap();

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

    let mut tiles = 0;
    match dijkstra(&grid, start, end) {
        Some((distance, path, slower)) => {
            // for x in &path {
            //     println!("{x:?}");
            // }
            //
            // for (key, values) in &slower {
            //     println!(
            //         "{}: {:?}",
            //         key.as_str().yellow(),
            //         values
            //             .iter()
            //             .map(|v| v.to_string().yellow())
            //             .collect::<Vec<_>>()
            //     );
            // }

            println!("Shortest distance: {distance}");
            for (row_idx, row) in grid.iter().enumerate() {
                for (col_idx, cell) in row.iter().enumerate() {
                    let mut printed = false;
                    for point in &path {
                        if point.0.x == row_idx as isize && point.0.y == col_idx as isize {
                            let directions = [">", "v", "<", "^"];
                            tiles += 1;
                            print!("{}", directions[point.1 as usize].red());
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

    println!("{tiles}");
}

fn dijkstra(
    grid: &[Vec<char>],
    start: Node,
    end: Point,
) -> Option<(i32, HashSet<Node>, HashMap<Node, Vec<Node>>)> {
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    let rows = grid.len();
    let cols = grid[0].len();

    let mut distances = HashMap::new();
    let mut heap = BinaryHeap::new();
    let mut paths = HashMap::new();
    let mut slower = HashMap::new();

    distances.insert(start, 0);
    heap.push(Reverse((0, start)));

    while let Some(Reverse((dist, node))) = heap.pop() {
        if node.0 == end {
            let mut best_tiles = HashSet::new();
            let mut stack = vec![node];

            // Backtrack through all predecessors to find all unique tiles
            while let Some(current) = stack.pop() {
                if !best_tiles.insert(current) {
                    continue; // Skip already-processed tiles
                }

                if let Some(parents) = paths.get(&current) {
                    for &x in parents {
                        stack.push(x);
                    }
                }
            }
            return Some((dist, best_tiles, slower));
        }

        for (new, weight) in get_adjacent_nodes(grid, node, &directions) {
            if new.0.x < 0 || new.0.y < 0 || new.0.x >= rows as isize || new.0.y >= cols as isize {
                continue;
            }

            let current_dist = *distances.get(&new).unwrap_or(&i32::MAX);
            let new_dist = dist + weight;
            if new_dist < current_dist {
                distances.insert(new, new_dist);
                paths.insert(new, vec![node]);
                heap.push(Reverse((new_dist, new)));
            } else if new_dist > current_dist {
                slower.entry(new).or_insert_with(Vec::new).push(node);
            } else if new_dist == current_dist {
                paths.get_mut(&new).unwrap().push(node);
            }
        }
    }
    None
}

fn get_adjacent_nodes(
    grid: &[Vec<char>],
    (current_node, direction): Node,
    directions: &[(isize, isize)],
) -> Vec<(Node, i32)> {
    let mut adjacent_nodes = Vec::new();
    let (nx, ny) = (
        current_node.x + directions[direction as usize].0,
        current_node.y + directions[direction as usize].1,
    );
    let neighbor = Point { x: nx, y: ny };

    if grid[neighbor.x as usize][neighbor.y as usize] != '#' {
        adjacent_nodes.push(((neighbor, direction), 1));
    }
    adjacent_nodes.push(((current_node, (direction + 3) % 4), 1000));
    adjacent_nodes.push(((current_node, (direction + 1) % 4), 1000));
    adjacent_nodes
}
