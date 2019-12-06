#![feature(slice_patterns)]

use std::iter;

fn main() {}

pub fn run_intcode(program: Vec<i32>, input: i32) -> Vec<i32> {
    iter::successors(Some((program, 0)), |(program, ip)| {
        process_opcode(&program, *ip, input)
    })
    .last()
    .unwrap()
    .0
}

fn process_opcode(program: &Vec<i32>, ip: usize, input: i32) -> Option<(Vec<i32>, usize)> {
    let opcode = format!("{:05}", program[ip]);
    let mut iter = opcode.chars();
    let _a = iter.next().unwrap();
    let b = iter.next().unwrap();
    let c = iter.next().unwrap();
    match &opcode.as_bytes()[3..5] {
        b"01" => {
            let x = if c == '0' {
                program[program[ip + 1] as usize]
            } else {
                program[ip + 1]
            };
            let y = if b == '0' {
                program[program[ip + 2] as usize]
            } else {
                program[ip + 2]
            };
            let z = program[ip + 3] as usize;
            let mut copy = program.clone();
            copy[z] = x + y;
            Some((copy, ip + 4))
        }
        b"02" => {
            let x = if c == '0' {
                program[program[ip + 1] as usize]
            } else {
                program[ip + 1]
            };
            let y = if b == '0' {
                program[program[ip + 2] as usize]
            } else {
                program[ip + 2]
            };
            let z = program[ip + 3] as usize;
            let mut copy = program.clone();
            copy[z] = x * y;
            Some((copy, ip + 4))
        }
        b"03" => {
            let x = program[ip + 1] as usize;
            let mut copy = program.clone();
            copy[x] = input;
            Some((copy, ip + 2))
        }
        b"04" => {
            let x = if c == '0' {
                program[program[ip + 1] as usize]
            } else {
                program[ip + 1]
            };
            println!("{}", x);
            Some((program.clone(), ip + 2))
        }
        b"05" => {
            let x = if c == '0' {
                program[program[ip + 1] as usize]
            } else {
                program[ip + 1]
            };
            if x != 0 {
                let y = if b == '0' {
                    program[program[ip + 2] as usize]
                } else {
                    program[ip + 2]
                } as usize;
                Some((program.clone(), y))
            } else {
                Some((program.clone(), ip + 3))
            }
        }
        b"06" => {
            let x = if c == '0' {
                program[program[ip + 1] as usize]
            } else {
                program[ip + 1]
            };
            if x == 0 {
                let y = if b == '0' {
                    program[program[ip + 2] as usize]
                } else {
                    program[ip + 2]
                } as usize;
                Some((program.clone(), y))
            } else {
                Some((program.clone(), ip + 3))
            }
        }
        b"07" => {
            let x = if c == '0' {
                program[program[ip + 1] as usize]
            } else {
                program[ip + 1]
            };
            let y = if b == '0' {
                program[program[ip + 2] as usize]
            } else {
                program[ip + 2]
            };
            let z = program[ip + 3] as usize;
            let mut copy = program.clone();
            if x < y {
                copy[z] = 1;
            } else {
                copy[z] = 0;
            }
            Some((copy, ip + 4))
        }
        b"08" => {
            let x = if c == '0' {
                program[program[ip + 1] as usize]
            } else {
                program[ip + 1]
            };
            let y = if b == '0' {
                program[program[ip + 2] as usize]
            } else {
                program[ip + 2]
            };
            let z = program[ip + 3] as usize;
            let mut copy = program.clone();
            if x == y {
                copy[z] = 1;
            } else {
                copy[z] = 0;
            }
            Some((copy, ip + 4))
        }
        b"99" => None,
        _ => panic!(),
    }
}
