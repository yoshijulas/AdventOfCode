use regex::Regex;

fn main() {
    let line = std::fs::read_to_string("./day3a/src/input.txt").unwrap();

    let re = Regex::new(r"don't\(\)|do\(\)|mul\(\d{1,3},\d{1,3}\)").unwrap();
    let re_sparated = Regex::new(r"\d{1,3}").unwrap();

    let matches: Vec<&str> = re.find_iter(&line).map(|m| m.as_str()).collect();

    let mut enabled = true;
    let mut sum = 0;

    for detection in matches {
        match detection {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            num => {
                if enabled {
                    let num: Vec<i32> = re_sparated
                        .find_iter(num)
                        .map(|m| m.as_str().parse().unwrap())
                        .collect();
                    sum += num[0] * num[1];
                }
            }
        }
    }

    print!("{sum}");
}
