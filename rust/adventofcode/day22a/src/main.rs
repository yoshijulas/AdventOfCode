use std::collections::HashMap;

#[derive(Default, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

fn main() {
    let input = std::fs::read_to_string("day22a/src/input.txt").unwrap();
    let secret_list: Vec<usize> = input.lines().map(|x| x.parse().unwrap()).collect();

    let mut sum = 0;
    let mut cache = HashMap::new();

    for start_secret in secret_list {
        sum += simulate(start_secret, 2000, &mut cache);
    }

    print!("{sum}");
}

fn calculate_secret(start: usize, num: usize, cache: &mut HashMap<(usize, usize), usize>) -> usize {
    // if let Some(&new_secret) = cache.get(&(start, num)) {
    //     return new_secret;
    // }
    let mut next = start;
    next = ((next * 64) ^ next) % 16_777_216;
    next = ((next / 32) ^ next) % 16_777_216;
    next = ((next * 2048) ^ next) % 16_777_216;

    // cache.insert((start, num), next);
    next
}

fn simulate(secret: usize, num: usize, cache: &mut HashMap<(usize, usize), usize>) -> usize {
    let mut secret = secret;
    for _ in 0..num {
        secret = calculate_secret(secret, num, cache);
    }
    secret
}
