fn main() {
    let lines = std::fs::read_to_string("./day2b/src/input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|number| number.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let safe_reports = lines
        .iter()
        .map(|line| {
            if is_safe(line) {
                true
            } else {
                (0..line.len()).any(|i| {
                    let mut modified_line = line.clone();
                    modified_line.remove(i); // Remove the ith level
                    is_safe(&modified_line)
                })
            }
        })
        .filter(|&lines| lines)
        .count();

    println!("{safe_reports}");
}

fn is_safe(line: &[i32]) -> bool {
    let mut trend = 0;
    line.windows(2).all(|pair| {
        let diff = (pair[0] - pair[1]).abs();

        if !(1..=3).contains(&diff) {
            return false;
        }

        if pair[0] < pair[1] {
            if trend == 0 {
                trend = 1;
            };
            trend == 1
        } else {
            if trend == 0 {
                trend = 2;
            };
            trend == 2
        }
    })
}
