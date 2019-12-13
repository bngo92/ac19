#![feature(slice_patterns)]

use std::iter;
use std::sync::mpsc::{Receiver, Sender};

fn main() {}

pub fn run_intcode(
    program: Vec<i64>,
    input: &Option<&mut Receiver<i64>>,
    output: &Option<&mut Sender<i64>>,
) -> Vec<i64> {
    iter::successors(Some((program, 0, 0)), |(program, ip, relative_base)| {
        process_opcode(program.clone(), *ip, *relative_base, input, output)
    })
    .last()
    .unwrap()
    .0
}

pub fn process_opcode(
    mut program: Vec<i64>,
    ip: usize,
    relative_base: usize,
    input: &Option<&mut Receiver<i64>>,
    output: &Option<&mut Sender<i64>>,
) -> Option<(Vec<i64>, usize, usize)> {
    //println!("{:?}", program);
    let opcode = format!("{:05}", program[ip]);
    //println!("{}, {:?}", ip, opcode);
    let mut iter = opcode.chars();
    let a = iter.next().unwrap();
    let b = iter.next().unwrap();
    let c = iter.next().unwrap();
    match &opcode.as_bytes()[3..5] {
        b"01" => {
            //println!("{} {:?}", opcode, &program[ip..ip + 4]);
            let x = get_value(&mut program, ip + 1, c, relative_base);
            let y = get_value(&mut program, ip + 2, b, relative_base);
            let z = get_address(&mut program, ip + 3, a, relative_base);
            grow(&mut program, z, 0);
            let next = [&program[..z], &program.get(z + 1..).unwrap_or(&[])].join(&(x + y));
            Some((next, ip + 4, relative_base))
        }
        b"02" => {
            //println!("{} {:?}", opcode, &program[ip..ip + 4]);
            let x = get_value(&mut program, ip + 1, c, relative_base);
            let y = get_value(&mut program, ip + 2, b, relative_base);
            let z = get_address(&mut program, ip + 3, a, relative_base);
            grow(&mut program, z, 0);
            let next = [&program[..z], &program.get(z + 1..).unwrap_or(&[])].join(&(x * y));
            Some((next, ip + 4, relative_base))
        }
        b"03" => {
            //println!("{} {:?}", opcode, &program[ip..ip + 4]);
            let i = input.as_ref().unwrap().recv().unwrap();
            //println!("{} received {}", std::thread::current().name().unwrap(), i);
            let x = get_address(&mut program, ip + 1, c, relative_base);
            let next = [&program[..x], &program[x + 1..]].join(&i);
            Some((next, ip + 2, relative_base))
        }
        b"04" => {
            //println!("{} {:?}", opcode, &program[ip..ip + 2]);
            let x = get_value(&mut program, ip + 1, c, relative_base);
            //println!("{} sent {}", std::thread::current().name().unwrap_or("thread"), x);
            output.as_ref().unwrap().send(x);
            Some((program.clone(), ip + 2, relative_base))
        }
        b"05" => {
            //println!("{} {:?}", opcode, &program[ip..ip + 3]);
            let x = get_value(&mut program, ip + 1, c, relative_base);
            if x != 0 {
                let y = get_value(&mut program, ip + 2, b, relative_base) as usize;
                Some((program.clone(), y, relative_base))
            } else {
                Some((program.clone(), ip + 3, relative_base))
            }
        }
        b"06" => {
            //println!("{} {:?}", opcode, &program[ip..ip + 3]);
            let x = get_value(&mut program, ip + 1, c, relative_base);
            if x == 0 {
                let y = get_value(&mut program, ip + 2, b, relative_base) as usize;
                Some((program.clone(), y, relative_base))
            } else {
                Some((program.clone(), ip + 3, relative_base))
            }
        }
        b"07" => {
            //println!("{} {:?}", opcode, &program[ip..ip + 4]);
            let x = get_value(&mut program, ip + 1, c, relative_base);
            let y = get_value(&mut program, ip + 2, b, relative_base);
            let z = get_address(&mut program, ip + 3, a, relative_base);
            let next = [&program[..z], &program[z + 1..]].join(&((x < y) as i64));
            Some((next, ip + 4, relative_base))
        }
        b"08" => {
            //println!("{} {:?}", opcode, &program[ip..ip + 4]);
            let x = get_value(&mut program, ip + 1, c, relative_base);
            let y = get_value(&mut program, ip + 2, b, relative_base);
            let z = get_address(&mut program, ip + 3, a, relative_base);
            let next = [&program[..z], &program[z + 1..]].join(&((x == y) as i64));
            Some((next, ip + 4, relative_base))
        }
        b"09" => {
            //println!("{} {:?}", opcode, &program[ip..ip + 2]);
            let x = get_value(&mut program, ip + 1, c, relative_base);
            Some((
                program.clone(),
                ip + 2,
                ((relative_base as i64) + x) as usize,
            ))
        }
        b"99" => None,
        _ => panic!(),
    }
}

fn get_value(mut program: &mut Vec<i64>, ip: usize, mode: char, relative_base: usize) -> i64 {
    let ip = get_address(program, ip, mode, relative_base);
    grow(&mut program, ip + 1, 0);
    program[ip]
}

fn get_address(program: &Vec<i64>, ip: usize, mode: char, relative_base: usize) -> usize {
    if mode == '0' {
        program[ip] as usize
    } else if mode == '2' {
        (relative_base as i64 + program[ip as usize]) as usize
    } else {
        ip
    }
}

fn grow(v: &mut Vec<i64>, new_len: usize, value: i64) {
    if new_len > v.len() {
        v.resize(new_len, value);
    }
}
