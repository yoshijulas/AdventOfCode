use std::collections::HashMap;

use crossterm::style::Stylize;

fn main() {
    let input = std::fs::read_to_string("./day19a/src/input.txt").unwrap();
    let input_split: Vec<&str> = input.split("\r\n\r\n").collect();
    let patterns: Vec<&str> = input_split[0].split(',').map(|x| x.trim()).collect();
    let designs: Vec<&str> = input_split[1].lines().collect();

    let mut posibles = 0;
    for design in designs {
        let mut memo = HashMap::new();
        posibles += calculate_pattern(String::new(), design, &patterns, &mut memo);
    }

    print!("{posibles}");
}

fn calculate_pattern(
    last_design: String,
    design: &str,
    patterns: &[&str],
    memo: &mut HashMap<String, usize>,
) -> usize {
    if let Some(&patterns) = memo.get(&last_design.to_string()) {
        return patterns;
    }

    if last_design == design {
        memo.insert(last_design, 1);
        return 1;
    }

    let mut total_ways = 0;

    for pattern in patterns {
        // Append the pattern to the current design
        let new_design = format!("{last_design}{pattern}");
        let new_len = new_design.len();

        // Check if the new design matches the corresponding prefix of the target
        if new_len <= design.len() && new_design == design[..new_len] {
            total_ways += calculate_pattern(new_design.to_string(), design, patterns, memo);
        }
    }

    memo.insert(last_design, total_ways);
    total_ways
}
