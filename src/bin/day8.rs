use std::env;
use std::fs;

fn main() {
    let args: Vec<_> = env::args().collect();
    let s = fs::read_to_string("day8.txt").unwrap();
    let s = s.trim();
    const X: usize = 25;
    const Y: usize = 6;
    let ls = day8(s, X, Y);
    if args[1] == "p1" {
        let l = ls
            .into_iter()
            .min_by_key(|s| s.iter().filter(|&&i| i == 0).count())
            .unwrap();
        println!(
            "{:?}",
            l.iter().filter(|&&i| i == 1).count() * l.iter().filter(|&&i| i == 2).count()
        );
    } else if args[1] == "p2" {
        println!(
            "{}",
            part2(ls, X, Y)
                .chars()
                .map(|p| match p {
                    '1' => '@',
                    '0' => ' ',
                    c => c,
                })
                .collect::<String>()
        );
    }
}

fn day8(s: &str, width: usize, length: usize) -> Vec<Vec<i32>> {
    s.split("")
        .filter_map(|s| s.parse::<i32>().ok())
        .collect::<Vec<_>>()
        .chunks(width * length)
        .map(<[i32]>::to_vec)
        .collect()
}

fn part2(ls: Vec<Vec<i32>>, width: usize, length: usize) -> String {
    String::from_utf8(
        (0..length)
            .map(|y| {
                (0..width)
                    .map(|x| {
                        ls.iter()
                            .find_map(|l| match l[y * width + x] {
                                1 => Some(b'1'),
                                0 => Some(b'0'),
                                _ => None,
                            })
                            .unwrap()
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
            .join(&b'\n'),
    )
    .unwrap()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            day8("123456789012", 3, 2),
            vec![vec![1, 2, 3, 4, 5, 6], vec![7, 8, 9, 0, 1, 2]]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            part2(day8("0222112222120000", 2, 2), 2, 2),
            "01
10"
        );
    }
}
