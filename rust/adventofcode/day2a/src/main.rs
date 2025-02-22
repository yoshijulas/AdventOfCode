fn main() {
    let lines = std::fs::read_to_string("./day2a/src/input.txt")
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
        })
        .filter(|&lines| lines)
        .count();

    println!("{safe_reports}");
}
