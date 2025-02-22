use crossterm::style::Stylize;
use regex::Regex;

struct Registers {
    a: usize,
    b: usize,
    c: usize,
}

fn main() {
    let input = std::fs::read_to_string("./day17b/src/input.txt").unwrap();
    let re = Regex::new(r"[0-9]+").unwrap();

    let input: Vec<&str> = input.split("\r\n\r\n").collect();

    let (_, instructions_unsplit) = (input[0], input[1]);

    let instructions: Vec<usize> = re
        .find_iter(instructions_unsplit)
        .map(|x| x.as_str().parse().unwrap())
        .collect();

    let res_a = search_solution(0, instructions.len() - 1, &instructions).unwrap();

    let mut regs = Registers {
        a: res_a,
        b: 0,
        c: 0,
    };
    let out = run_program(&instructions, &mut regs);
    let expected = instructions
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join(",");

    assert_eq!(expected, out);
    println!("{res_a}");
}

fn search_solution(a_prev: usize, i: usize, instructions: &Vec<usize>) -> Option<usize> {
    for j in 0..(1 << 3) {
        let next_a = (a_prev << 3) + j;
        let expected = instructions[i..]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let mut regs = Registers {
            a: next_a,
            b: 0,
            c: 0,
        };

        if expected == run_program(instructions, &mut regs) {
            if i == 0 {
                return Some(next_a);
            }

            if let Some(a) = search_solution(next_a, i - 1, instructions) {
                return Some(a);
            }
        }
    }

    None
}

fn run_program(instructions: &[usize], register: &mut Registers) -> String {
    let mut ip = 0;
    let mut out = Vec::new();

    loop {
        let opcode = instructions[ip];
        let operand = instructions[ip + 1];

        match opcode {
            0 => register.a /= 1 << get_combo_num(operand, register),
            1 => register.b ^= operand,
            2 => register.b = get_combo_num(operand, register) % 8,
            3 => {
                if register.a != 0 {
                    ip = operand;
                    continue;
                }
            }
            4 => register.b ^= register.c,
            5 => {
                (get_combo_num(operand, register) % 8)
                    .to_string()
                    .chars()
                    .for_each(|x| out.push(x));
            }
            6 => register.b = register.a / (1 << get_combo_num(operand, register)),
            7 => register.c = register.a / (1 << get_combo_num(operand, register)),
            _ => {}
        }
        ip += 2;
        if ip >= instructions.len() {
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
