use std::collections::HashMap;
fn main() {
    let line: Vec<usize> = std::fs::read_to_string("./day11a/src/input.txt")
        .unwrap()
        .split_whitespace()
        .map(|x| x.trim().parse::<usize>().unwrap())
        .collect();

    let mut cache = HashMap::new();
    let mut res = 0;
    for rocks in line {
        res += change_rocks(rocks, 2000, &mut cache);
    }

    print!("{res}");
}

fn change_rocks(rock: usize, rounds: usize, cache: &mut HashMap<(usize, usize), usize>) -> usize {
    // Recursion anchor
    if rounds == 0 {
        return 1;
    }
    // Calculation is already cached
    if let Some(res) = cache.get(&(rock, rounds)) {
        return *res;
    }

    // Recurse
    let res = match rock {
        0 => change_rocks(1, rounds - 1, cache),
        n => {
            let digits = rock.ilog10() + 1;
            if digits % 2 == 0 {
                let divisor = 10usize.pow(digits / 2);
                change_rocks(n / divisor, rounds - 1, cache)
                    + change_rocks(n % divisor, rounds - 1, cache)
            } else {
                change_rocks(n * 2024, rounds - 1, cache)
            }
        }
    };

    // Save in cache
    cache.insert((rock, rounds), res);
    res
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
