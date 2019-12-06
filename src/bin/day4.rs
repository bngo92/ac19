use std::collections::{HashMap, HashSet};
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args[1] == "p1" {
        println!(
            "{}",
            (172930..683082)
                .map(|i| i.to_string())
                .filter(|s| s.as_bytes().windows(2).all(|s| match s {
                    [c1, c2] => c1 <= c2,
                    _ => panic!(),
                }))
                .filter(|s| s.as_bytes().windows(2).any(|s| match s {
                    [c1, c2] => c1 == c2,
                    _ => panic!(),
                }))
                .count()
        );
    } else if args[1] == "p2" {
        println!(
            "{}",
            (172930..683082)
                .map(|i| i.to_string())
                .filter(|s| s.as_bytes().windows(2).all(|s| match s {
                    [c1, c2] => c1 <= c2,
                    _ => panic!(),
                }))
                .filter(|s| {
                    let mut counts = HashMap::new();
                    s.bytes().for_each(|c| {
                        *counts.entry(c).or_insert(0) += 1;
                    });
                    counts.values().collect::<HashSet<_>>().contains(&2)
                })
                .count()
        );
    }
}
