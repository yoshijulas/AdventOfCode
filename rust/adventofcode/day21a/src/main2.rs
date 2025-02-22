use std::collections::{HashMap, VecDeque};

use crossterm::style::Stylize;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

struct Keypad {
    pad: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
    start_position: Point,
}

impl Point {
    fn apply_direction(&self, (dx, dy): (isize, isize), rows: usize, cols: usize) -> Option<Point> {
        let new_x = self.x as isize + dx;
        let new_y = self.y as isize + dy;

        if new_x >= 0 && new_x < rows as isize && new_y >= 0 && new_y < cols as isize {
            Some(Point {
                x: new_x as usize,
                y: new_y as usize,
            })
        } else {
            None
        }
    }
}

const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

fn main() {
    let input = std::fs::read_to_string("./day21a/src/input.txt").unwrap();

    let keycodes: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();

    // let start = find_values(&grid, 'S');

    #[rustfmt::skip]
    let num_keypad = Keypad {
        pad: vec![
            vec!['7', '8', '9'],
            vec!['4', '5', '6'],
            vec!['1', '2', '3'],
            vec!['#', '0', 'A'],
        ],
        rows: 4,
        cols: 3,
        start_position: Point { x: 3, y: 2 }
    };

    #[rustfmt::skip]
    let dir_keypad = Keypad {
        pad: vec![
            vec!['#', '^', 'A'], 
            vec!['<', 'v', '>']],
        rows: 2,
        cols: 3,
        start_position: Point { x: 0, y: 2 }
    };

    let kp_combi_one = calculate_combination(&num_keypad, &keycodes);
    print_combi(&kp_combi_one, &keycodes);

    let kp_comb_two = calculate_combination(&dir_keypad, &kp_combi_one);
    print_combi(&kp_comb_two, &keycodes);

    let kp_comb_three = calculate_combination(&dir_keypad, &kp_comb_two);
    print_combi(&kp_comb_three, &keycodes);

    let mut sum = 0;

    for (idx, keycode) in keycodes.iter().enumerate() {
        let number: usize = keycode
            .iter()
            .filter(|&&x| x != 'A')
            .collect::<String>()
            .parse()
            .unwrap();
        let len = kp_comb_three[idx].len();
        sum += number * len;
        println!("{len} {number}");
    }

    print!("{sum}");

    //
    // for way in &keypad_combo_two {
    //     print!("{:<4}: ", "2");
    //     for x in way {
    //         print!("{x}");
    //     }
    //     println!();
    // }
}

fn print_combi(keypad_combination: &[Vec<char>], keycodes: &[Vec<char>]) {
    for (idx, way) in keypad_combination.iter().enumerate() {
        print!("{}: ", keycodes[idx].iter().collect::<String>());
        for x in way {
            print!("{x}");
        }
        println!();
    }
}

fn calculate_combination(keypad: &Keypad, keycodes: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut next_combination = Vec::new();
    let mut last_keypad = keypad.start_position;

    for key in keycodes {
        let mut row = Vec::new();
        for &k in key {
            let end_point = find_values(&keypad.pad, k);
            let mut path = bfs(keypad, last_keypad, end_point);
            if !path.is_empty() {
                path.push('A'); // Add the activation press after reaching the target
            }
            row.extend(path);
            last_keypad = end_point;
        }
        next_combination.push(row);
    }
    next_combination
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

fn find_shortest_path(
    keypad: &Keypad,
    current: Point,
    target: Point,
    memo: &mut HashMap<(Point, Point), Option<Vec<char>>>,
) -> Option<Vec<char>> {
    // Check memo first
    if let Some(path) = memo.get(&(current, target)) {
        return path.clone();
    }

    // Base case: we reached the target
    if current == target {
        return Some(Vec::new());
    }

    let mut shortest_path = None;
    let mut min_length = usize::MAX;

    // Try each direction
    for &dir in &DIRECTIONS {
        if let Some(next) = current.apply_direction(dir, keypad.rows, keypad.cols) {
            // Skip walls and avoid going in circles
            if keypad.pad[next.x][next.y] == '#' {
                continue;
            }

            // Recursive call
            if let Some(mut path) = find_shortest_path(keypad, next, target, memo) {
                path.insert(0, direction_from_points(current, next));

                if path.len() < min_length {
                    min_length = path.len();
                    shortest_path = Some(path);
                }
            }
        }
    }

    // Store result in memo before returning
    memo.insert((current, target), shortest_path.clone());
    shortest_path
}

fn direction_from_points(start: Point, end: Point) -> char {
    match (
        end.x as isize - start.x as isize,
        end.y as isize - start.y as isize,
    ) {
        (1, 0) => 'v',
        (0, 1) => '>',
        (-1, 0) => '^',
        (0, -1) => '<',
        _ => panic!("Invalid direction"),
    }
}
