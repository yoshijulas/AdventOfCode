use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("day22a/src/input.txt").unwrap();
    let secret_list: Vec<usize> = input.lines().map(|x| x.parse().unwrap()).collect();
    let bananas = simulate(secret_list);
    println!("{bananas}");
}

const fn calculate_secret(start: usize) -> usize {
    let mut next = start;
    next = ((next * 64) ^ next) % 16_777_216;
    next = ((next / 32) ^ next) % 16_777_216;
    next = ((next * 2048) ^ next) % 16_777_216;
    next
}

fn simulate(secret_list: Vec<usize>) -> usize {
    let mut totals = HashMap::new();

    for initial_value in secret_list {
        let mut deltas = String::with_capacity(2000);
        let mut values = Vec::with_capacity(2000);
        let mut prev = initial_value;

        // Calculate 2000 steps for this initial value
        for _ in 0..2000 {
            let next = calculate_secret(prev);
            values.push(next % 10);
            // Create character representation of the difference
            deltas.push((10u8 + b'A' + ((prev % 10) as u8) - ((next % 10) as u8)) as char);
            prev = next;
        }

        // Store patterns of length 4 and their corresponding banana counts
        let mut cache = HashMap::new();
        for j in 0..deltas.len() - 4 {
            let pattern = &deltas[j..j + 4];
            let bananas = values[j + 3];
            cache.entry(pattern.to_string()).or_insert(bananas);
        }

        // Add up banana counts for each pattern across all sequences
        for (key, value) in cache {
            *totals.entry(key).or_insert(0) += value;
        }
    }

    *totals.values().max().unwrap()
}
