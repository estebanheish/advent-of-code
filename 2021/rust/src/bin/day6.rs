use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/day6.txt").unwrap();

    let input: Vec<usize> = input
        .strip_suffix("\n")
        .unwrap()
        .split(",")
        .map(|p| p.parse().unwrap())
        .collect();

    let mut peces: [usize; 9] = input.iter().fold([0; 9], |mut acc, &n| {
        acc[n] += 1;
        acc
    });

    for _ in 1..=256 {
        let [cero, u, d, t, c, f, seis, siete, ocho] = peces;
        peces = [u, d, t, c, f, seis, siete + cero, ocho, cero];
    }

    println!("{}", peces.iter().sum::<usize>());
}
