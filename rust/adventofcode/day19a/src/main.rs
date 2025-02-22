use crossterm::style::Stylize;

fn main() {
    let input = std::fs::read_to_string("./day19a/src/input.txt").unwrap();
    let input_split: Vec<&str> = input.split("\r\n\r\n").collect();
    let patterns: Vec<&str> = input_split[0].split(',').map(|x| x.trim()).collect();
    let designs: Vec<&str> = input_split[1].lines().collect();

    let mut posibles = 0;
    for design in designs {
        if calculate_pattern("", design, &patterns).is_some() {
            posibles += 1;
        }
    }

    print!("{posibles}");
}

fn calculate_pattern(last_design: &str, design: &str, patterns: &[&str]) -> Option<()> {
    if last_design == design {
        return Some(());
    }

    for pattern in patterns {
        // Append the pattern to the current design
        let new_design = format!("{last_design}{pattern}");
        let new_len = new_design.len();

        // Check if the new design matches the corresponding prefix of the target
        if new_len <= design.len()
            && new_design == design[..new_len]
            && calculate_pattern(&new_design, design, patterns).is_some()
        {
            return Some(());
        }
    }

    None
}
