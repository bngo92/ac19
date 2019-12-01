mod d1;

fn main() {
    println!("{}", S { state: 14 }.sum::<i32>());
    println!("{}", S { state: 1969 }.sum::<i32>());
    println!("{}", S { state: 100756 }.sum::<i32>());
    let i: i32 = d1::input()
        .into_iter()
        .map(|s| S { state: s }.sum::<i32>())
        .sum();
    println!("{}", i);
}

struct S {
    state: i32,
}

impl Iterator for S {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        self.state = d1::d1(self.state);
        if self.state > 0 {
            Some(self.state)
        } else {
            None
        }
    }
}
