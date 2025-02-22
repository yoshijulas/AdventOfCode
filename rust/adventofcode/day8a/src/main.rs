use std::collections::{HashMap, HashSet};

#[derive(Hash, Eq, PartialEq, Debug)]
struct Position {
    x: isize,
    y: isize,
}

fn main() {
    let line = std::fs::read_to_string("./day8a/src/input.txt").unwrap();

    let mut input: Vec<Vec<char>> = line.lines().map(|line| line.chars().collect()).collect();

    let mut coords: HashMap<char, Vec<Position>> = HashMap::new();

    for (row_idx, rows) in input.iter().enumerate() {
        for (col_idx, &cell) in rows.iter().enumerate() {
            if cell != '.' {
                coords.entry(cell).or_default().push(Position {
                    x: row_idx as isize,
                    y: col_idx as isize,
                });
            }
        }
    }

    let mut unique_positions = HashSet::new();

    for (freq, antenna_set) in &coords {
        for i in 0..antenna_set.len() {
            for j in (1 + i)..antenna_set.len() {
                let antenna1 = &antenna_set[i];
                let antenna2 = &antenna_set[j];
                if input[antenna1.x as usize][antenna1.y as usize]
                    == input[antenna2.x as usize][antenna2.y as usize]
                {
                    let antinode = calculate_distance_between_antennas(antenna1, antenna2);
                    for node in antinode {
                        // dbg!(&node);
                        if (node.x as usize) < input.len() && (node.y as usize) < input[0].len() {
                            unique_positions.insert(node);
                        }
                    }
                }
            }
        }
    }
    for col in &input {
        for cell in col {
            print!("{cell}");
        }
        println!();
    }
    for position in &unique_positions {
        println!("{position:?}");
        input[position.x as usize][position.y as usize] = '#';
    }

    println!();
    for col in input {
        for cell in col {
            print!("{cell}");
        }
        println!();
    }

    print!("{}", unique_positions.len());

    //
    // for x in coords {
    //     for y in x.1 {
    //         println!("{0} {y:?}", x.0);
    //     }
    // }
    // println!("{input:?}");
}

fn calculate_distance_between_antennas(antenna1: &Position, antenna2: &Position) -> Vec<Position> {
    let slope: f32 = (antenna2.y - antenna1.y) as f32 / (antenna2.x - antenna1.x) as f32;
    // println!("{antenna1:?}");
    // println!("{antenna2:?}");
    // println!("slope {slope}");

    let (smaller, bigger) = if antenna1.x < antenna2.x {
        (antenna1, antenna2)
    } else {
        (antenna2, antenna1)
    };
    calculate_all_antinode(slope, smaller, bigger)
}

fn calculate_all_antinode(slope: f32, antenna1: &Position, antenna2: &Position) -> Vec<Position> {
    let delta_x = antenna2.x - antenna1.x;

    let x1_prime = antenna1.x - delta_x;

    let y1_prime: f32 = antenna1.y as f32 - (slope * delta_x as f32);

    let x2_prime = antenna2.x + delta_x;

    let y2_prime: f32 = antenna2.y as f32 + (slope * delta_x as f32);

    let mut include_1 = true;

    let mut include_2 = true;

    if x1_prime < 0 || y1_prime < 0.0 {
        include_1 = false;
    }
    if x2_prime < 0 || y2_prime < 0.0 {
        include_2 = false;
    }

    let mut antinodes = Vec::new();

    if include_1 {
        antinodes.push(Position {
            x: x1_prime,
            y: y1_prime.round() as isize,
        });
    }
    if include_2 {
        antinodes.push(Position {
            x: x2_prime,
            y: y2_prime.round() as isize,
        });
    }

    antinodes
}
