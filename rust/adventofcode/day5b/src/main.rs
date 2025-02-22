use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

fn main() {
    let (page_rules, page_order) = process_input();

    let mut dependencies: HashMap<i32, HashSet<i32>> = HashMap::new();

    // Split into x | y, y, y, y
    for (x, y) in page_rules {
        dependencies.entry(x).or_default().insert(y);
    }

    let mut unordered_pages = Vec::new();

    for order in page_order {
        if !is_valid_order(&order, &dependencies) {
            unordered_pages.push(order);
        }
    }

    let mut sum_of_now_ordered = 0;

    for unordered in unordered_pages {
        sum_of_now_ordered += order_invalid_pages(&unordered, &dependencies);
    }

    print!("{sum_of_now_ordered}");
}

fn process_input() -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let line = std::fs::read_to_string("./day5b/src/input.txt").unwrap();

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
fn order_invalid_pages(order: &[i32], dependencies: &HashMap<i32, HashSet<i32>>) -> i32 {
    let mut new_order = order.to_vec();

    for i in 0..new_order.len() {
        for j in i + 1..new_order.len() {
            let a = new_order[i];
            let b = new_order[j];

            if let Some(dep) = dependencies.get(&a) {
                if dep.contains(&b) {
                    new_order.swap(i, j);
                }
            }
        }
    }

    new_order[new_order.len() / 2]
}

// fn order_invalid_pages(order: &[i32], dependencies: &HashMap<i32, HashSet<i32>>) -> i32 {
//     let mut new_order = order.to_vec();
//
//     new_order.sort_by(|a, b| {
//         if dependencies.get(a).is_some_and(|x| x.contains(b)) {
//             Ordering::Less
//         } else {
//             Ordering::Greater
//         }
//     });
//
//     new_order[new_order.len() / 2]
// }
