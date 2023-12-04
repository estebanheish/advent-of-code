use std::{
    collections::HashSet,
    io::{stdin, Read},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let input = input.trim();

    let parsed: Vec<(HashSet<usize>, HashSet<usize>)> = input
        .lines()
        .map(|l| {
            let (w, h) = l.split_once(":").unwrap().1.split_once("|").unwrap();
            (
                w.split_whitespace().map(|s| s.parse().unwrap()).collect(),
                h.split_whitespace().map(|s| s.parse().unwrap()).collect(),
            )
        })
        .collect();

    let matching: Vec<usize> = parsed
        .iter()
        .map(|(w, h)| w.intersection(h).count())
        .collect();

    println!(
        "part 1 -> {}",
        matching
            .iter()
            .map(|n| (1 * (2_usize.pow(*n as u32))) / 2)
            .sum::<usize>()
    );

    let mut have = vec![0; matching.len()];
    for (i, mn) in matching.iter().enumerate() {
        have[i] += 1;
        for j in i + 1..i + 1 + mn {
            have[j] += 1 * have[i];
        }
    }

    println!("part 2 -> {}", have.iter().sum::<usize>());
}
