#![feature(slice_patterns, vec_remove_item)]

use std::env;
use std::sync::mpsc::channel;
use std::thread;

mod intcode;

fn main() {
    let args: Vec<_> = env::args().collect();
    let my: Vec<_> = "3,8,1001,8,10,8,105,1,0,0,21,46,59,80,105,122,203,284,365,446,99999,3,9,102,3,9,9,1001,9,5,9,102,2,9,9,1001,9,3,9,102,4,9,9,4,9,99,3,9,1002,9,2,9,101,2,9,9,4,9,99,3,9,101,5,9,9,1002,9,3,9,1001,9,3,9,1002,9,2,9,4,9,99,3,9,1002,9,4,9,1001,9,2,9,102,4,9,9,101,3,9,9,102,2,9,9,4,9,99,3,9,102,5,9,9,101,4,9,9,102,3,9,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,1,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,99,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,99,3,9,102,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,1,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,99,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,99"
            .to_owned()
            .split(',')
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
    if args[1] == "p1" {
        let s: Vec<_> = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"
            .to_owned()
            .split(',')
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        part1(s);
        let s: Vec<_> = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"
            .to_owned()
            .split(',')
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        part1(s);
        let s: Vec<_> = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"
            .to_owned()
            .split(',')
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        part1(s);
        part1(my.clone());
    } else if args[1] == "p2" {
        let s: Vec<_> =
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"
                .to_owned()
                .split(',')
                .map(|s| s.parse::<i64>().unwrap())
                .collect();
        println!("{}", part2(s.clone(), [9, 8, 7, 6, 5]));
        p2(s);
        let s: Vec<_> =
            "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"
                .to_owned()
                .split(',')
                .map(|s| s.parse::<i64>().unwrap())
                .collect();
        println!("{}", part2(s.clone(), [9, 7, 8, 5, 6]));
        p2(s);
        p2(my);
    }
}

fn part1(s: Vec<i64>) {
    let range: Vec<_> = (0..5).collect();
    let mut max = 0;
    for a in &range {
        let (output, mut a_input) = channel();
        output.send(*a).unwrap();
        output.send(0).unwrap();
        let (mut a_output, output) = channel();
        intcode::run_intcode(s.clone(), &Some(&mut a_input), &Some(&mut a_output));
        let a_output: Vec<i64> = output.iter().collect();
        let mut range1 = range.clone();
        range1.remove_item(&a);
        for b in &range1 {
            let (input, mut b_input) = channel();
            for i in &a_output {
                input.send(*i);
            }
            let (mut b_output, output) = channel();
            intcode::run_intcode(s.clone(), &Some(&mut b_input), &Some(&mut b_output));
            let b_output: Vec<i64> = output.iter().collect();
            let mut range2 = range1.clone();
            range2.remove_item(&b);
            for c in &range2 {
                let (input, mut c_input) = channel();
                for i in &b_output {
                    input.send(*i);
                }
                let (mut c_output, output) = channel();
                intcode::run_intcode(s.clone(), &Some(&mut c_input), &Some(&mut c_output));
                let c_output: Vec<i64> = output.iter().collect();
                let mut range3 = range2.clone();
                range3.remove_item(&c);
                for d in &range3 {
                    let (input, mut d_input) = channel();
                    for i in &c_output {
                        input.send(*i);
                    }
                    let (mut d_output, output) = channel();
                    intcode::run_intcode(s.clone(), &Some(&mut d_input), &Some(&mut d_output));
                    let d_output: Vec<i64> = output.iter().collect();
                    let mut range4 = range3.clone();
                    range4.remove_item(&d);
                    let e = range4[0];
                    let (input, mut e_input) = channel();
                    for i in &d_output {
                        input.send(*i);
                    }
                    let (mut e_output, output) = channel();
                    intcode::run_intcode(s.clone(), &Some(&mut e_input), &Some(&mut e_output));
                    let o = output.recv().unwrap();
                    if o > max {
                        println!("{:?}: {}", [*a, *b, *c, *d, e], o);
                        max = o;
                    }
                }
            }
        }
    }
}

fn p2(s: Vec<i64>) {
    let range: Vec<_> = (5..10).collect();
    let mut max = 0;
    for a in &range {
        let mut range1 = range.clone();
        range1.remove_item(&a);
        for b in &range1 {
            let mut range2 = range1.clone();
            range2.remove_item(&b);
            for c in &range2 {
                let mut range3 = range2.clone();
                range3.remove_item(&c);
                for d in &range3 {
                    let mut range4 = range3.clone();
                    range4.remove_item(&d);
                    let e = range4[0];
                    let o = part2(s.clone(), [*a, *b, *c, *d, e]);
                    if o > max {
                        println!("{:?}: {}", [*a, *b, *c, *d, e], o);
                        max = o;
                    }
                }
            }
        }
    }
}
fn part2(program: Vec<i64>, input: [i64; 5]) -> i64 {
    let [a, b, c, d, e] = input;
    let (mut e_output, mut a_input) = channel();
    e_output.send(a).unwrap();
    e_output.send(0).unwrap();
    let mut amp_a = (program.clone(), 0, 0);
    let (mut a_output, mut b_input) = channel();
    a_output.send(b).unwrap();
    let mut amp_b = (program.clone(), 0, 0);
    let (mut b_output, mut c_input) = channel();
    b_output.send(c).unwrap();
    let mut amp_c = (program.clone(), 0, 0);
    let (mut c_output, mut d_input) = channel();
    c_output.send(d).unwrap();
    let mut amp_d = (program.clone(), 0, 0);
    let (mut d_output, mut e_input) = channel();
    d_output.send(e).unwrap();
    let mut amp_e = (program.clone(), 0, 0);
    let t1 = thread::Builder::new()
        .name("a".to_string())
        .spawn(move || {
            loop {
                match intcode::process_opcode(
                    amp_a.0,
                    amp_a.1,
                    0,
                    &Some(&mut a_input),
                    &Some(&mut a_output),
                ) {
                    Some(t) => {
                        amp_a = t;
                    }
                    None => {
                        break;
                    }
                };
            }
            a_input.recv().unwrap()
        })
        .unwrap();
    thread::Builder::new()
        .name("b".to_string())
        .spawn(move || loop {
            match intcode::process_opcode(
                amp_b.0,
                amp_b.1,
                0,
                &Some(&mut b_input),
                &Some(&mut b_output),
            ) {
                Some(t) => {
                    amp_b = t;
                }
                None => {
                    break;
                }
            }
        });
    thread::Builder::new()
        .name("c".to_string())
        .spawn(move || loop {
            match intcode::process_opcode(
                amp_c.0,
                amp_c.1,
                0,
                &Some(&mut c_input),
                &Some(&mut c_output),
            ) {
                Some(t) => {
                    amp_c = t;
                }
                None => {
                    break;
                }
            }
        });
    thread::Builder::new()
        .name("d".to_string())
        .spawn(move || loop {
            match intcode::process_opcode(
                amp_d.0,
                amp_d.1,
                0,
                &Some(&mut d_input),
                &Some(&mut d_output),
            ) {
                Some(t) => {
                    amp_d = t;
                }
                None => {
                    break;
                }
            }
        });
    thread::Builder::new()
        .name("e".to_string())
        .spawn(move || loop {
            match intcode::process_opcode(
                amp_e.0,
                amp_e.1,
                0,
                &Some(&mut e_input),
                &Some(&mut e_output),
            ) {
                Some(t) => {
                    amp_e = t;
                }
                None => {
                    break;
                }
            }
        })
        .unwrap();
    t1.join().unwrap()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            intcode::run_intcode(
                "1,9,10,3,2,3,11,0,99,30,40,50"
                    .to_owned()
                    .split(',')
                    .map(|s| s.parse::<i64>().unwrap())
                    .collect(),
                &None,
                &None,
            ),
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            intcode::run_intcode(
                "1,0,0,0,99"
                    .to_owned()
                    .split(',')
                    .map(|s| s.parse::<i64>().unwrap())
                    .collect(),
                &None,
                &None,
            ),
            vec![2, 0, 0, 0, 99]
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            intcode::run_intcode(
                "2,3,0,3,99"
                    .to_owned()
                    .split(',')
                    .map(|s| s.parse::<i64>().unwrap())
                    .collect(),
                &None,
                &None,
            ),
            vec![2, 3, 0, 6, 99]
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            intcode::run_intcode(
                "2,4,4,5,99,0"
                    .to_owned()
                    .split(',')
                    .map(|s| s.parse::<i64>().unwrap())
                    .collect(),
                &None,
                &None,
            ),
            vec![2, 4, 4, 5, 99, 9801]
        );
    }

    #[test]
    fn test_5() {
        assert_eq!(
            intcode::run_intcode(
                "1,1,1,4,99,5,6,0,99"
                    .to_owned()
                    .split(',')
                    .map(|s| s.parse::<i64>().unwrap())
                    .collect(),
                &None,
                &None,
            ),
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
        );
    }
}
