mod d1;

fn main() {
    println!("{}", part2(14));
    println!("{}", part2(1969));
    println!("{}", part2(100756));
    let i: i32 = d1::input().into_iter().map(part2).sum();
    println!("{}", i);
}

fn part2(i: i32) -> i32 {
    let mut i = i;
    let mut acc = 0;
    loop {
        let fuel = d1::d1(i);
        if fuel <= 0 {
            break
        }
        acc += fuel;
        i = fuel;
    }
    acc
}
