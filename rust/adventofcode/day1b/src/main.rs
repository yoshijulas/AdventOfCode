use std::collections::HashMap;

fn main() {
    let (left, right): (Vec<i32>, Vec<i32>) = std::fs::read_to_string("./day1b/src/input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|number| number.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|numbers| (numbers[0], numbers[1]))
        .unzip();

    let mut times_num: HashMap<i32, i32> = HashMap::new();

    for item_right in right {
        times_num
            .entry(item_right)
            .and_modify(|x| *x += 1)
            .or_insert(1);
    }

    let sum = left
        .iter()
        .map(|&item_left| item_left * times_num.get(&item_left).unwrap_or(&0))
        .sum::<i32>();

    println!("{sum}");
}
