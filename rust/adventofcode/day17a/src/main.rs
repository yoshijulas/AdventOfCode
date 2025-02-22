use crossterm::style::Stylize;
use regex::Regex;

struct Registers {
    a: usize,
    b: usize,
    c: usize,
}

fn main() {
    let input = std::fs::read_to_string("./day17a/src/input.txt").unwrap();
    let re = Regex::new(r"[0-9]+").unwrap();

    let input: Vec<&str> = input.split("\r\n\r\n").collect();

    let (registers, instructions) = (input[0], input[1]);

    let registers_split: Vec<usize> = re
        .find_iter(registers)
        .map(|x| x.as_str().parse().unwrap())
        .collect();

    let instructions_split: Vec<usize> = re
        .find_iter(instructions)
        .map(|x| x.as_str().parse().unwrap())
        .collect();

    let mut register = Registers {
        a: registers_split[0],
        b: registers_split[1],
        c: registers_split[2],
    };

    let out = run_program(instructions_split, register);

    print!("{out}");
}

fn run_program(instructions_split: Vec<usize>, mut register: Registers) -> String {
    let mut ip = 0;
    let mut out = Vec::new();

    loop {
        let opcode = instructions_split[ip];
        let operand = instructions_split[ip + 1];

        match opcode {
            0 => register.a /= 1 << get_combo_num(operand, &register),
            1 => register.b ^= operand,
            2 => register.b = get_combo_num(operand, &register) % 8,
            3 => {
                if register.a != 0 {
                    ip = operand;
                    continue;
                }
            }
            4 => register.b ^= register.c,
            5 => {
                (get_combo_num(operand, &register) % 8)
                    .to_string()
                    .chars()
                    .for_each(|x| out.push(x));
            }
            6 => register.b /= 1 << get_combo_num(operand, &register),
            7 => register.c /= 1 << get_combo_num(operand, &register),

            _ => {}
        }
        ip += 2;
        if ip >= instructions_split.len() {
            break;
        }
    }

    out.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn get_combo_num(operand: usize, register: &Registers) -> usize {
    match operand {
        0..=3 => operand,
        4 => register.a,
        5 => register.b,
        6 => register.c,
        7 => panic!("Should not be used"),
        _ => unreachable!(),
    }
}
