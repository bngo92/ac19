#![feature(slice_patterns)]

use std::collections::VecDeque;
use std::iter;

fn main() {}

pub fn run_intcode(program: Vec<i32>, mut input: VecDeque<i32>) -> (Vec<i32>, VecDeque<i32>) {
    let mut output = VecDeque::new();
    (
        iter::successors(Some((program, 0)), |(program, ip)| {
            match process_opcode(&program, *ip, &mut input, &mut output) {
                OpcodeResult::Next(t) => Some(t),
                OpcodeResult::Halt => None,
                _ => panic!(),
            }
        })
        .last()
        .unwrap()
        .0,
        output,
    )
}

pub enum OpcodeResult {
    Next((Vec<i32>, usize)),
    Halt,
    Blocked,
}

pub fn process_opcode(
    program: &Vec<i32>,
    ip: usize,
    input: &mut VecDeque<i32>,
    output: &mut VecDeque<i32>,
) -> OpcodeResult {
    //println!("{:?}", program);
    let opcode = format!("{:05}", program[ip]);
    //println!("{}, {:?}", ip, opcode);
    let mut iter = opcode.chars();
    let _a = iter.next().unwrap();
    let b = iter.next().unwrap();
    let c = iter.next().unwrap();
    match &opcode.as_bytes()[3..5] {
        b"01" => {
            let x = get_value(program, ip + 1, c);
            let y = get_value(program, ip + 2, b);
            let z = program[ip + 3] as usize;
            let next = [&program[..z], &program[z + 1..]].join(&(x + y));
            OpcodeResult::Next((next, ip + 4))
        }
        b"02" => {
            let x = get_value(program, ip + 1, c);
            let y = get_value(program, ip + 2, b);
            let z = program[ip + 3] as usize;
            let next = [&program[..z], &program[z + 1..]].join(&(x * y));
            OpcodeResult::Next((next, ip + 4))
        }
        b"03" => match input.pop_front() {
            Some(i) => {
                let x = program[ip + 1] as usize;
                let next = [&program[..x], &program[x + 1..]].join(&i);
                OpcodeResult::Next((next, ip + 2))
            }
            None => OpcodeResult::Blocked,
        },
        b"04" => {
            let x = get_value(program, ip + 1, c);
            output.push_back(x);
            OpcodeResult::Next((program.clone(), ip + 2))
        }
        b"05" => {
            let x = get_value(program, ip + 1, c);
            if x != 0 {
                let y = get_value(program, ip + 2, b) as usize;
                OpcodeResult::Next((program.clone(), y))
            } else {
                OpcodeResult::Next((program.clone(), ip + 3))
            }
        }
        b"06" => {
            let x = get_value(program, ip + 1, c);
            if x == 0 {
                let y = get_value(program, ip + 2, b) as usize;
                OpcodeResult::Next((program.clone(), y))
            } else {
                OpcodeResult::Next((program.clone(), ip + 3))
            }
        }
        b"07" => {
            let x = get_value(program, ip + 1, c);
            let y = get_value(program, ip + 2, b);
            let z = program[ip + 3] as usize;
            let next = [&program[..z], &program[z + 1..]].join(&((x < y) as i32));
            OpcodeResult::Next((next, ip + 4))
        }
        b"08" => {
            let x = get_value(program, ip + 1, c);
            let y = get_value(program, ip + 2, b);
            let z = program[ip + 3] as usize;
            let next = [&program[..z], &program[z + 1..]].join(&((x == y) as i32));
            OpcodeResult::Next((next, ip + 4))
        }
        b"99" => OpcodeResult::Halt,
        _ => panic!(),
    }
}

fn get_value(program: &Vec<i32>, ip: usize, mode: char) -> i32 {
    if mode == '0' {
        program[program[ip] as usize]
    } else {
        program[ip]
    }
}
