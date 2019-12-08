#![feature(slice_patterns, vec_remove_item)]

use std::env;

mod intcode;

fn main() {
    let args: Vec<_> = env::args().collect();
    let my: Vec<_> = "3,8,1001,8,10,8,105,1,0,0,21,46,59,80,105,122,203,284,365,446,99999,3,9,102,3,9,9,1001,9,5,9,102,2,9,9,1001,9,3,9,102,4,9,9,4,9,99,3,9,1002,9,2,9,101,2,9,9,4,9,99,3,9,101,5,9,9,1002,9,3,9,1001,9,3,9,1002,9,2,9,4,9,99,3,9,1002,9,4,9,1001,9,2,9,102,4,9,9,101,3,9,9,102,2,9,9,4,9,99,3,9,102,5,9,9,101,4,9,9,102,3,9,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,1,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,99,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,99,3,9,102,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,1,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,99,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,99"
            .to_owned()
            .split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
    if args[1] == "p1" {
        let s: Vec<_> = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"
            .to_owned()
            .split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        part1(s);
        let s: Vec<_> = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"
            .to_owned()
            .split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        part1(s);
        let s: Vec<_> = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"
            .to_owned()
            .split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        part1(s);
        part1(my.clone());
    } else if args[1] == "p2" {
        let s: Vec<_> =
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"
                .to_owned()
                .split(',')
                .map(|s| s.parse::<i32>().unwrap())
                .collect();
        part2(s.clone(), [9, 8, 7, 6, 5]);
        p2(s);
        let s: Vec<_> =
            "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"
                .to_owned()
                .split(',')
                .map(|s| s.parse::<i32>().unwrap())
                .collect();
        part2(s.clone(), [9, 7, 8, 5, 6]);
        p2(s);
        p2(my);
    }
}

fn part1(s: Vec<i32>) {
    let range: Vec<_> = (0..5).collect();
    let mut max = 0;
    for a in &range {
        let o1 = *intcode::run_intcode(s.clone(), vec![*a, 0].into_iter().collect())
            .1
            .get(0)
            .unwrap();
        let mut range1 = range.clone();
        range1.remove_item(&a);
        for b in &range1 {
            let o2 = *intcode::run_intcode(s.clone(), vec![*b, o1].into_iter().collect())
                .1
                .get(0)
                .unwrap();
            let mut range2 = range1.clone();
            range2.remove_item(&b);
            for c in &range2 {
                let o3 = *intcode::run_intcode(s.clone(), vec![*c, o2].into_iter().collect())
                    .1
                    .get(0)
                    .unwrap();
                let mut range3 = range2.clone();
                range3.remove_item(&c);
                for d in &range3 {
                    let o4 = *intcode::run_intcode(s.clone(), vec![*d, o3].into_iter().collect())
                        .1
                        .get(0)
                        .unwrap();
                    let mut range4 = range3.clone();
                    range4.remove_item(&d);
                    let e = range4[0];
                    let o = *intcode::run_intcode(s.clone(), vec![e, o4].into_iter().collect())
                        .1
                        .get(0)
                        .unwrap();
                    if o > max {
                        println!("{:?}: {}", [*a, *b, *c, *d, e], o);
                        max = o;
                    }
                }
            }
        }
    }
}

fn p2(s: Vec<i32>) {
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
fn part2(program: Vec<i32>, input: [i32; 5]) -> i32 {
    let [a, b, c, d, e] = input;
    let mut amp_a_input = vec![a, 0].into_iter().collect();
    let mut amp_a = (program.clone(), 0);
    let mut a_running = true;
    let mut amp_b_input = vec![b].into_iter().collect();
    let mut amp_b = (program.clone(), 0);
    let mut b_running = true;
    let mut amp_c_input = vec![c].into_iter().collect();
    let mut amp_c = (program.clone(), 0);
    let mut c_running = true;
    let mut amp_d_input = vec![d].into_iter().collect();
    let mut amp_d = (program.clone(), 0);
    let mut d_running = true;
    let mut amp_e_input = vec![e].into_iter().collect();
    let mut amp_e = (program.clone(), 0);
    loop {
        while a_running {
            match intcode::process_opcode(&amp_a.0, amp_a.1, &mut amp_a_input, &mut amp_b_input) {
                intcode::OpcodeResult::Next(t) => {
                    amp_a = t;
                }
                intcode::OpcodeResult::Blocked => {
                    break;
                }
                intcode::OpcodeResult::Halt => {
                    a_running = false;
                }
            }
        }
        //println!("b {:?}", amp_b_input);
        while b_running {
            match intcode::process_opcode(&amp_b.0, amp_b.1, &mut amp_b_input, &mut amp_c_input) {
                intcode::OpcodeResult::Next(t) => {
                    amp_b = t;
                }
                intcode::OpcodeResult::Blocked => {
                    break;
                }
                intcode::OpcodeResult::Halt => {
                    b_running = false;
                }
            }
        }
        //println!("c {:?}", amp_c_input);
        while c_running {
            match intcode::process_opcode(&amp_c.0, amp_c.1, &mut amp_c_input, &mut amp_d_input) {
                intcode::OpcodeResult::Next(t) => {
                    amp_c = t;
                }
                intcode::OpcodeResult::Blocked => {
                    break;
                }
                intcode::OpcodeResult::Halt => {
                    c_running = false;
                }
            }
        }
        //println!("d {:?}", amp_d_input);
        while d_running {
            match intcode::process_opcode(&amp_d.0, amp_d.1, &mut amp_d_input, &mut amp_e_input) {
                intcode::OpcodeResult::Next(t) => {
                    amp_d = t;
                }
                intcode::OpcodeResult::Blocked => {
                    break;
                }
                intcode::OpcodeResult::Halt => {
                    d_running = false;
                }
            }
        }
        //println!("e {:?}", amp_e_input);
        loop {
            match intcode::process_opcode(&amp_e.0, amp_e.1, &mut amp_e_input, &mut amp_a_input) {
                intcode::OpcodeResult::Next(t) => {
                    amp_e = t;
                }
                intcode::OpcodeResult::Blocked => {
                    break;
                }
                intcode::OpcodeResult::Halt => {
                    return *amp_a_input.get(0).unwrap();
                }
            }
        }
        //println!("a {:?}", amp_a_input);
    }
}

/*#[cfg(test)]
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
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect(),
                -1
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
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect(),
                -1
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
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect(),
                -1
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
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect(),
                -1
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
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect(),
                -1
            ),
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
        );
    }
}*/
