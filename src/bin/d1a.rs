mod d1;

fn main() {
    println!("{}", d1::d1(12));
    println!("{}", d1::d1(14));
    println!("{}", d1::d1(1969));
    println!("{}", d1::d1(100756));
    let i: i32 = d1::input().into_iter().map(d1::d1).sum();
    println!("{}", i);
}
