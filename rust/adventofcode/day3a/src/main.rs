use regex::Regex;

fn main() {
    let line = std::fs::read_to_string("./day3a/src/input.txt").unwrap();

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let answer = re
        .captures_iter(&line)
        .map(|c| {
            let a = c.get(1).unwrap().as_str();
            let b = c.get(2).unwrap().as_str();
            (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap())
        })
        .fold(0, |acc, num| acc + (num.0 * num.1));

    print!("{answer}");
}
