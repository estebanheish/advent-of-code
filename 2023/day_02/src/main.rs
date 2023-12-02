use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input = input.trim();

    let parsed: Vec<Vec<(u32, u32, u32)>> = input
        .lines()
        .map(|l| {
            l.split_once(":")
                .unwrap()
                .1
                .split(";")
                .map(|subset| {
                    let mut rgb = (0, 0, 0);
                    subset.split(",").for_each(|cubes| {
                        match cubes.trim().split_once(" ").unwrap() {
                            (n, "red") => rgb.0 = n.parse().unwrap(),
                            (n, "green") => rgb.1 = n.parse().unwrap(),
                            (n, "blue") => rgb.2 = n.parse().unwrap(),
                            _ => (),
                        }
                    });
                    rgb
                })
                .collect()
        })
        .collect();

    println!("{parsed:?}");

    println!(
        "part 1 -> {}",
        parsed
            .iter()
            .enumerate()
            .filter(|(_, set)| { set.iter().all(|(r, g, b)| *r <= 12 && *g <= 13 && *b <= 14) })
            .map(|(i, _)| i + 1)
            .sum::<usize>()
    );

    println!(
        "part 2 -> {}",
        parsed
            .iter()
            .map(|set| {
                let (r, g, b) = set.iter().fold((0, 0, 0), |acc, new| {
                    (acc.0.max(new.0), acc.1.max(new.1), acc.2.max(new.2))
                });
                r * g * b
            })
            .sum::<u32>()
    );
}
