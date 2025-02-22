use std::collections::{HashMap, HashSet};

fn main() {
    let (page_rules, page_order) = process_input();

    let mut dependencies: HashMap<i32, HashSet<i32>> = HashMap::new();

    // Split into x | y, y, y, y
    for (x, y) in page_rules {
        dependencies.entry(x).or_default().insert(y);
    }

    let mut sum = 0;

    for order in page_order {
        if is_valid_order(&order, &dependencies) {
            let middle_idx = order.len() / 2;
            sum += order[middle_idx];
        }
    }

    print!("{sum}");
}

fn process_input() -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let line = std::fs::read_to_string("./day5a/src/input.txt").unwrap();

    let input: Vec<&str> = line.lines().collect();

    let split_idx = input.iter().position(|x| x.is_empty()).unwrap();

    let page_rules: Vec<(i32, i32)> = input[..split_idx]
        .iter()
        .map(|x| {
            let parts: Vec<i32> = x.split('|').map(|x| x.parse().unwrap()).collect();
            (parts[0], parts[1])
        })
        .collect();

    let page_order: Vec<Vec<i32>> = input[split_idx + 1..]
        .iter()
        .map(|s| s.split(',').map(|x| x.parse().unwrap()).collect())
        .collect();
    (page_rules, page_order)
}

/// checks if pages are printed in order, checking if the current pages has no page printed before
/// it
///
/// * `order`: Line to verify if its in order
/// * `dependencies`: Rules to see if is printed in order
fn is_valid_order(order: &[i32], dependencies: &HashMap<i32, HashSet<i32>>) -> bool {
    for (idx, current) in order.iter().enumerate() {
        if let Some(required_after) = dependencies.get(current) {
            for must_be_after in required_after {
                if order[..idx].contains(must_be_after) {
                    return false;
                }
            }
        }
    }
    true
}
