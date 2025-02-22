fn main() {
    let (mut left, mut right): (Vec<i32>, Vec<i32>) =
        std::fs::read_to_string("./day1b/src/input.txt")
            .unwrap()
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|number| number.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            })
            .map(|numbers| (numbers[0], numbers[1]))
            .unzip();

    left.sort_unstable();
    right.sort_unstable();

    let numbers = left
        .iter()
        .zip(right.iter())
        .fold(0, |acc, (a, b)| acc + (a - b).abs());

    println!("{numbers}");
}
