fn main() {
    let line = std::fs::read_to_string("./day7a/src/input.txt").unwrap();

    let input: Vec<(i64, Vec<i64>)> = line
        .lines()
        .map(|line| {
            let mut parts = line.split(':');
            let key = parts.next().unwrap().parse().unwrap();
            let value = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            (key, value)
        })
        .collect();

    let mut sum = 0;
    for num in input {
        let number = calculate_number(num.1[0], &num.1[1..]);
        if number.contains(&num.0) {
            sum += num.0;
        }
    }

    print!("{sum}");

    // println!("{input:?}");
}

fn calculate_number(current: i64, numbers: &[i64]) -> Vec<i64> {
    if numbers.is_empty() {
        vec![current]
    } else {
        let first = numbers[0];
        let rest = &numbers[1..];
        let mut result = Vec::new();

        result.extend(calculate_number(current + first, rest));
        result.extend(calculate_number(current * first, rest));
        result.extend(calculate_number(
            format!("{current}{first}").parse().unwrap(),
            rest,
        ));
        result
    }
}
