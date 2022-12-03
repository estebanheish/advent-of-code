#![feature(iter_array_chunks)]

fn main() {
    let input = include_str!("../../input.txt");
    println!("{}\n{}", part1(input), part2(input));
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .flat_map(|r| {
            let (a, b) = r.split_at(r.len() / 2);
            a.chars().filter(|&c| b.contains(c)).take(1).map(rank)
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .array_chunks::<3>()
        .flat_map(|[x, y, z]| {
            x.chars()
                .filter(|&c| y.contains(c) && z.contains(c))
                .take(1)
                .map(rank)
        })
        .sum()
}

fn rank(c: char) -> u32 {
    if c.is_lowercase() {
        (c as u8 - b'a' + 1) as u32
    } else {
        (c as u8 - b'A' + 27) as u32
    }
}
