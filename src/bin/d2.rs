use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    let s: Vec<_> = "1,12,2,3,1,1,2,3,1,3,4,3,1,5,0,3,2,10,1,19,1,19,9,23,1,23,13,27,1,10,27,31,2,31,13,35,1,10,35,39,2,9,39,43,2,43,9,47,1,6,47,51,1,10,51,55,2,55,13,59,1,59,10,63,2,63,13,67,2,67,9,71,1,6,71,75,2,75,9,79,1,79,5,83,2,83,13,87,1,9,87,91,1,13,91,95,1,2,95,99,1,99,6,0,99,2,14,0,0".to_owned().split(',').map(|s| s.parse::<i32>().unwrap()).collect();
    if args[1] == "p1" {
        println!("{:?}", d2("1,0,0,0,99".to_owned().split(',').map(|s| s.parse::<i32>().unwrap()).collect()));
        println!("{:?}", d2("2,3,0,3,99".to_owned().split(',').map(|s| s.parse::<i32>().unwrap()).collect()));
        println!("{:?}", d2("2,4,4,5,99,0".to_owned().split(',').map(|s| s.parse::<i32>().unwrap()).collect()));
        println!("{:?}", d2("1,1,1,4,99,5,6,0,99".to_owned().split(',').map(|s| s.parse::<i32>().unwrap()).collect()));
        println!("{:?}", d2("1,9,10,3,2,3,11,0,99,30,40,50".to_owned().split(',').map(|s| s.parse::<i32>().unwrap()).collect()));
        println!("{:?}", d2("1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,10,1,19,1,19,9,23,1,23,13,27,1,10,27,31,2,31,13,35,1,10,35,39,2,9,39,43,2,43,9,47,1,6,47,51,1,10,51,55,2,55,13,59,1,59,10,63,2,63,13,67,2,67,9,71,1,6,71,75,2,75,9,79,1,79,5,83,2,83,13,87,1,9,87,91,1,13,91,95,1,2,95,99,1,99,6,0,99,2,14,0,0".to_owned().split(',').map(|s| s.parse::<i32>().unwrap()).collect()));
        println!("{:?}", d2(s));
    } else if args[1] == "p2" {
        for noun in 0..=99 {
           for verb in 0..=99 {
               let mut copy = s.clone();
               copy[1] = noun;
               copy[2] = verb;
               if d2(copy)[0] == 19690720 {
                   println!("{}", 100 * noun + verb);
                   break;
               }
           }
        }
    }
}

fn d2(input: Vec<i32>) -> Vec<i32> {
    let mut pos = 0;
    let mut input = input;
    while let Some(next) = d2_next(&input, pos) {
        input = next;
        pos += 4;
    }
    input
}

fn d2_next(input: &Vec<i32>, position: usize) -> Option<Vec<i32>> {
    match input[position] {
        1 => {
            let x = input[position + 1] as usize;
            let y = input[position + 2] as usize;
            let z = input[position + 3] as usize;
            let mut copy = input.clone();
            copy[z] = input[x] + input[y];
            Some(copy)
        }
        2 => {
            let x = input[position + 1] as usize;
            let y = input[position + 2] as usize;
            let z = input[position + 3] as usize;
            let mut copy = input.clone();
            copy[z] = input[x] * input[y];
            Some(copy)
        }
        99 => None,
        _ => panic!()
    }
}
