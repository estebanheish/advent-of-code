use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input = input.trim();

    println!(
        "part 1 -> {}",
        input
            .lines()
            .map(|l| {
                let mut it = l.chars();
                let f = it
                    .find(|c| c.is_ascii_digit())
                    .unwrap()
                    .to_digit(10)
                    .unwrap();
                let l = it.rev().find(|c| c.is_ascii_digit());
                f * 10
                    + if let Some(n) = l {
                        n.to_digit(10).unwrap()
                    } else {
                        f
                    }
            })
            .sum::<u32>()
    );
    println!(
        "part 2 -> {}",
        input
            .lines()
            .map(|l| {
                let l: Vec<char> = l.chars().collect();
                let mut d = Vec::new();
                for i in 0..l.len() {
                    if let Some(n) = m(&l[i..]) {
                        d.push(n);
                    }
                }
                d.first().unwrap() * 10 + d.last().unwrap()
            })
            .sum::<u32>()
    );
}

fn m(s: &[char]) -> Option<u32> {
    match s {
        ['o', 'n', 'e', ..] => Some(1),
        ['t', 'w', 'o', ..] => Some(2),
        ['t', 'h', 'r', 'e', 'e', ..] => Some(3),
        ['f', 'o', 'u', 'r', ..] => Some(4),
        ['f', 'i', 'v', 'e', ..] => Some(5),
        ['s', 'i', 'x', ..] => Some(6),
        ['s', 'e', 'v', 'e', 'n', ..] => Some(7),
        ['e', 'i', 'g', 'h', 't', ..] => Some(8),
        ['n', 'i', 'n', 'e', ..] => Some(9),
        [n, ..] if n.is_ascii_digit() => Some(n.to_digit(10).unwrap()),
        [_, ..] => None,
        [] => None,
    }
}
