fn find(input: &str, n: usize) -> usize {
    input
        .chars()
        .collect::<Vec<char>>()
        .windows(n)
        .position(|c| !(0..n).any(|i| (0..n).any(|y| c[i] == c[y] && y != i)))
        .unwrap()
        + n
}

fn main() {
    let input = include_str!("../../input.txt");
    println!("{}\n{}", find(input, 4), find(input, 14));
}
