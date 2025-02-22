use regex::Regex;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

struct Robot {
    p: Point,
    v: Point,
}

// impl Robot {
//     fn pretty_print(&self) {
//         print!("P: {} {} ", self.p.x, self.p.y);
//         print!("V: {} {} ", self.v.x, self.v.y);
//     }
// }

fn main() {
    let re = Regex::new(r"([0-9]+,[0-9]+) v=(\-?[0-9]+,\-?[0-9]+)").unwrap();
    let grid = std::fs::read_to_string("./day14a/src/input.txt").unwrap();

    let idk: Vec<Robot> = grid
        .trim()
        .lines()
        .flat_map(|line| {
            re.captures_iter(line).map(|c| {
                let a: Vec<i32> = c
                    .get(1)
                    .unwrap()
                    .as_str()
                    .split(',')
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect();
                let b: Vec<i32> = c
                    .get(2)
                    .unwrap()
                    .as_str()
                    .split(',')
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect();
                Robot {
                    p: Point { x: a[0], y: a[1] },
                    v: Point { x: b[0], y: b[1] },
                }
            })
        })
        .collect();

    let wide = 101;
    let tall = 103;

    let mut final_positions = Vec::new();

    for x in idk {
        let new_position = calculate_new_position(&x, 100, wide, tall);
        final_positions.push(new_position);
    }
    let sum = split_and_count_quadrant(final_positions, wide, tall);

    print!("{sum}");
}

const fn calculate_new_position(robot: &Robot, sec: i32, wide: i32, tall: i32) -> Point {
    let new_x = robot.p.x + (robot.v.x * (sec));
    let new_y = robot.p.y + (robot.v.y * (sec));

    let final_x = new_x.rem_euclid(wide);
    let final_y = new_y.rem_euclid(tall);

    Point {
        x: final_x,
        y: final_y,
    }
}

fn split_and_count_quadrant(final_positions: Vec<Point>, wide: i32, tall: i32) -> i32 {
    let half_wide = wide / 2;
    let half_tall = tall / 2;

    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;
    for position in final_positions {
        if position.x < half_wide && position.y < half_tall {
            q2 += 1;
        } else if position.x > half_wide && position.y < half_tall {
            q1 += 1;
        } else if position.x < half_wide && position.y > half_tall {
            q3 += 1;
        } else if position.x > half_wide && position.y > half_tall {
            q4 += 1;
        }
    }

    println!("{q1} {q2} {q3} {q4}");
    q1 * q2 * q3 * q4
}
