use std::collections::{HashMap, HashSet};
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args[1] == "p1" {
        println!(
            "{}",
            (172930..683082)
                .map(|i| i.to_string())
                .filter(|s| Pairwise::from_iter(&mut s.bytes()).all(|(c1, c2)| c1 <= c2))
                .filter(|s| Pairwise::from_iter(&mut s.bytes()).any(|(c1, c2)| c1 == c2))
                .count()
        );
    } else if args[1] == "p2" {
        println!(
            "{}",
            (172930..683082)
                .map(|i| i.to_string())
                .filter(|s| Pairwise::from_iter(&mut s.bytes()).all(|(c1, c2)| c1 <= c2))
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

struct Pairwise<'a> {
    iter: &'a mut dyn Iterator<Item = u8>,
    state: u8,
}

impl Pairwise<'_> {
    fn from_iter(iter: &'_ mut dyn Iterator<Item = u8>) -> Pairwise {
        let state = iter.next().unwrap();
        Pairwise { iter, state }
    }
}

impl Iterator for Pairwise<'_> {
    type Item = (u8, u8);

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.iter.next()?;
        let curr = self.state;
        self.state = next;
        Some((curr, next))
    }
}
