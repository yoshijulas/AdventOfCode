use regex::Regex;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

struct Robot {
    pos: Point,
    vel: Point,
}

fn main() {
    let re = Regex::new(r"([0-9]+,[0-9]+) v=(\-?[0-9]+,\-?[0-9]+)").unwrap();
    let grid = std::fs::read_to_string("./day14a/src/input.txt").unwrap();

    let robots: Vec<Robot> = grid
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
                    pos: Point { x: a[0], y: a[1] },
                    vel: Point { x: b[0], y: b[1] },
                }
            })
        })
        .collect();

    let wide = 101;
    let tall = 103;

    let mut closest_x_time = 0;
    let mut closest_y_time = 0;
    let mut closest_time = 0;

    let mut min_x_distance = f64::MAX;
    let mut min_y_distance = f64::MAX;
    let mut min_distance = f64::MAX;

    for time in 0..=100_000 {
        let positions: Vec<Point> = robots
            .iter()
            .map(|robot| calculate_new_position(robot, time, wide, tall))
            .collect();

        let avg_x_distance = calculate_avg_distance(&positions, wide / 2, true);
        let avg_y_distance = calculate_avg_distance(&positions, tall / 2, false);

        let sum_avg = (avg_y_distance + avg_x_distance) / 2.0;

        if sum_avg < min_distance {
            min_distance = sum_avg;
            closest_time = time;
        }

        if avg_x_distance < min_x_distance {
            min_x_distance = avg_x_distance;
            closest_x_time = time;
        }

        if avg_y_distance < min_y_distance {
            min_y_distance = avg_y_distance;
            closest_y_time = time;
        }
    }

    println!("Closest x to center at time {closest_x_time} with avg distance {min_x_distance}");
    println!("Closest y to center at time {closest_y_time} with avg distance {min_y_distance}");
    println!("Closest time {closest_time} with avg distance {min_distance}");

    let mut new_position = Vec::new();

    for robot in &robots {
        new_position.push(calculate_new_position(robot, closest_time, wide, tall));
    }

    visualize_positions(&new_position, wide, tall);
}

const fn calculate_new_position(robot: &Robot, sec: i32, wide: i32, tall: i32) -> Point {
    let new_x = robot.pos.x + (robot.vel.x * (sec));
    let new_y = robot.pos.y + (robot.vel.y * (sec));

    let final_x = new_x.rem_euclid(wide);
    let final_y = new_y.rem_euclid(tall);

    Point {
        x: final_x,
        y: final_y,
    }
}

fn split_and_count_quadrant(final_positions: Vec<Point>, wide: i32, tall: i32) -> i32 {
    let hwide = wide / 2;
    let htall = tall / 2;

    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;
    for pos in final_positions {
        if pos.x < hwide && pos.y < htall {
            q2 += 1;
        } else if pos.x > hwide && pos.y < htall {
            q1 += 1;
        } else if pos.x < hwide && pos.y > htall {
            q3 += 1;
        } else if pos.x > hwide && pos.y > htall {
            q4 += 1;
        }
    }

    println!("{q1} {q2} {q3} {q4}");
    q1 * q2 * q3 * q4
}

fn calculate_avg_distance(positions: &[Point], center: i32, is_x: bool) -> f64 {
    let total_distance: i32 = positions
        .iter()
        .map(|p| {
            if is_x {
                (p.x - center).abs()
            } else {
                (p.y - center).abs()
            }
        })
        .sum();
    total_distance as f64 / positions.len() as f64
}

fn visualize_positions(positions: &Vec<Point>, wide: i32, tall: i32) {
    let mut grid = vec![vec!['.'; wide as usize]; tall as usize];

    for p in positions {
        grid[p.y as usize][p.x as usize] = '#';
    }

    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
}
