use std::collections::HashMap;

#[derive(Default, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

struct Keypad {
    pad: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
    start_position: Point,
}

fn main() {
    let input = std::fs::read_to_string("day21a/src/input.txt").unwrap();
    let keycodes: Vec<&str> = input.lines().collect();

    #[rustfmt::skip]
    let num_keypad = Keypad {
        pad: vec![
            vec!['7', '8', '9'],
            vec!['4', '5', '6'],
            vec!['1', '2', '3'],
            vec!['#', '0', 'A'],
        ],
        rows: 4,
        cols: 3,
        start_position: Point::new(3, 2)
    };

    #[rustfmt::skip]
    let dir_keypad = Keypad {
        pad: vec![
            vec!['#', '^', 'A'], 
            vec!['<', 'v', '>']],
        rows: 2,
        cols: 3,
        start_position: Point::new(0, 2)
    };
    let mut total = 0;
    let depth = 25;
    for key in keycodes {
        let mut cache = HashMap::new();
        let min = calculate_codes(depth, key, &num_keypad, &dir_keypad, &mut cache);
        total += min * key[0..3].parse::<usize>().unwrap();
    }

    print!("{total}");
}

fn find_ch(keypad: &Keypad, ch: char) -> Point {
    for (row_idx, row) in keypad.pad.iter().enumerate() {
        for (col_idx, &cell) in row.iter().enumerate() {
            if cell == ch {
                return Point::new(row_idx, col_idx);
            }
        }
    }
    Point::default()
}

fn calculate_codes(
    depth: usize,
    keycode: &str,
    num_keypad: &Keypad,
    dir_keypad: &Keypad,
    cache: &mut HashMap<(String, usize), usize>,
) -> usize {
    let mut start = num_keypad.start_position;
    let mut length = 0;

    for next in keycode.chars() {
        let next_point = find_ch(num_keypad, next);
        let mut min = usize::MAX;
        for m in moves(num_keypad, start, next_point) {
            let len = calculate_dir_keypad(&m, depth, cache, dir_keypad);
            if len < min {
                min = len;
            }
        }
        length += min;
        start = next_point;
    }

    length
}

fn calculate_dir_keypad(
    keycode: &str,
    depth: usize,
    cache: &mut HashMap<(String, usize), usize>,
    kp: &Keypad,
) -> usize {
    if depth == 0 {
        return keycode.len();
    }

    if let Some(&length) = cache.get(&(keycode.to_string(), depth)) {
        return length;
    }

    let mut start = kp.start_position;
    let mut length = 0;

    for next in keycode.chars() {
        let next_point = find_ch(kp, next);
        let mut min = usize::MAX;
        for m in moves(kp, start, next_point) {
            let len = calculate_dir_keypad(&m, depth - 1, cache, kp);
            if len < min {
                min = len;
            }
        }
        length += min;
        start = next_point;
    }

    cache.insert((keycode.to_string(), depth), length);
    length
}

fn valid_path(keypad: &Keypad, start: Point, keycode: &str) -> bool {
    let mut pos = start;
    for c in keycode.chars() {
        match c {
            '^' => pos.x -= 1,
            'v' => pos.x += 1,
            '<' => pos.y -= 1,
            '>' => pos.y += 1,
            'A' => {}
            _ => unreachable!(),
        };

        if keypad.pad[pos.x][pos.y] == '#' {
            return false;
        }
    }
    true
}

fn moves(keypad: &Keypad, start: Point, end: Point) -> Vec<String> {
    let (delta_x, delta_y) = (
        (end.x as isize - start.x as isize),
        (end.y as isize - start.y as isize),
    );

    let vert = match delta_x {
        -3 => "^^^",
        -2 => "^^",
        -1 => "^",
        0 => "",
        1 => "v",
        2 => "vv",
        3 => "vvv",
        _ => unreachable!(),
    };

    let horz = match delta_y {
        -2 => "<<",
        -1 => "<",
        0 => "",
        1 => ">",
        2 => ">>",
        _ => unreachable!(),
    };

    let vert_path = horz.to_string() + vert + "A";
    let horz_path = vert.to_string() + horz + "A";

    if !valid_path(keypad, start, &vert_path) {
        return vec![horz_path];
    }
    if !valid_path(keypad, start, &horz_path) {
        return vec![vert_path];
    }
    vec![vert_path, horz_path]
}
