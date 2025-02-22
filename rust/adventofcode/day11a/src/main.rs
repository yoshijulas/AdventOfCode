use std::collections::VecDeque;

fn main() {
    let line: Vec<usize> = std::fs::read_to_string("./day11a/src/input.txt")
        .unwrap()
        .split_whitespace()
        .map(|x| x.trim().parse::<usize>().unwrap())
        .collect();

    // pretty_print("First", &line);

    // let size = line.len() * 2 * 75;

    let mut rocks: VecDeque<usize> = VecDeque::from(line);

    for i in 0..75 {
        println!("{i}");
        change_rocks(&mut rocks);
    }

    print!("{}", rocks.len());
}

fn change_rocks(line: &mut VecDeque<usize>) {
    let length = line.len();
    for _ in 0..length {
        if let Some(rock) = line.pop_front() {
            let digit_count = rock.checked_ilog10().unwrap_or(0) + 1;
            match rock {
                0 => line.push_back(1),
                x if digit_count % 2 == 0 => {
                    let split_point = digit_count / 2;
                    let divisor = 10_usize.pow(digit_count - split_point);
                    line.push_back(x / divisor);
                    line.push_back(x % divisor);
                }
                x => line.push_back(x * 2024),
            }
        }
    }
}

// fn pretty_print<T>(info: &str, arr: &[T])
// where
//     T: std::fmt::Display,
// {
//     print!("{info} ");
//     for col in arr {
//         print!("{col} ");
//     }
//     println!();
// }
