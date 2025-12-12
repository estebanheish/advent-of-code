use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let (shapes, trees) = input.trim().rsplit_once("\n\n").unwrap();
    let shapes: Vec<usize> = shapes
        .split("\n\n")
        .map(|s| {
            s.split_once(':')
                .unwrap()
                .1
                .trim()
                .lines()
                .map(|l| l.chars().filter(|c| *c == '#').count())
                .sum()
        })
        .collect();
    let trees: Vec<((usize, usize), Vec<usize>)> = trees
        .lines()
        .map(|t| {
            let (size, n_shapes) = t.split_once(':').unwrap();
            let (r, c) = size.split_once('x').unwrap();
            let n_shapes = n_shapes
                .split_ascii_whitespace()
                .map(|d| d.parse().unwrap())
                .collect();
            ((r.parse().unwrap(), c.parse().unwrap()), n_shapes)
        })
        .collect();

    println!(
        "{}",
        trees
            .into_iter()
            .filter(|(size, s_i)| {
                size.0 * size.1
                    >= s_i
                        .iter()
                        .enumerate()
                        .map(|(i, n)| n * shapes[i])
                        .sum::<usize>()
            })
            .count()
    );
}
