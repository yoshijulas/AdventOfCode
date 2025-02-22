use regex::Regex;

#[derive(Debug)]
struct Coordinates {
    x: i128,
    y: i128,
}

#[derive(Debug)]
struct GameCoords {
    button_a: Coordinates,
    button_b: Coordinates,
    prize: Coordinates,
}

impl GameCoords {
    const fn parse(a: &[i128], b: &[i128], res: &[i128]) -> Self {
        Self {
            button_a: Coordinates { x: a[0], y: a[1] },
            button_b: Coordinates { x: b[0], y: b[1] },
            prize: Coordinates {
                x: res[0],
                y: res[1],
            },
        }
    }

    fn pretty_print(self) {
        println!("A x: {}, y: {}", self.button_a.x, self.button_a.y);
        println!("B x: {}, y: {}", self.button_b.x, self.button_b.y);
        println!("R x: {}, y: {}", self.prize.x, self.prize.y);
    }
}

fn main() {
    let re = Regex::new(r"[0-9]+").unwrap();
    let grid = std::fs::read_to_string("./day13a/src/input.txt").unwrap();

    let idk: Vec<Vec<i128>> = grid
        .trim()
        .lines()
        .map(|line| {
            re.find_iter(line)
                .map(|x| x.as_str().parse::<i128>().unwrap())
                .collect()
        })
        .filter(|vec: &Vec<i128>| !vec.is_empty())
        .collect();

    let games: Vec<GameCoords> = idk
        .chunks(3)
        .map(|x| GameCoords::parse(&x[0], &x[1], &x[2]))
        .collect();

    let mut total = 0;

    for game in games {
        total += calculate_equation(&game);
        // game.pretty_print();
        // println!();
    }

    print!("{total}");
    // pretty_print("First", &line);
}

const fn calculate_equation(game: &GameCoords) -> i128 {
    // Calculate the determinant
    let det = game.button_a.x * game.button_b.y - game.button_b.x * game.button_a.y;

    // If determinant is 0, there is no solution
    if det == 0 {
        return 0;
    }

    // Replace a with prize and calculate determinant
    let det_a = game.prize.x * game.button_b.y - game.button_b.x * game.prize.y;
    // Replace b with prize and calculate determinant
    let det_b = game.prize.y * game.button_a.x - game.button_a.y * game.prize.x;

    let (a, a_rem) = (det_a / det, det_a % det);
    let (b, b_rem) = (det_b / det, det_b % det);

    // Check if number is integer
    if a_rem != 0 || b_rem != 0 {
        return 0;
    }

    // a value is 3 times, b is 1 time
    a * 3 + b
}
