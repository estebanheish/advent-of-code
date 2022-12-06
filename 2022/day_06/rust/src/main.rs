fn main() {
    let input = include_str!("../../input.txt");

    let first = input
        .chars()
        .collect::<Vec<char>>()
        .windows(4)
        .position(|c| !(0..4).any(|i| (0..4).any(|y| c[i] == c[y] && y != i)))
        .unwrap()
        + 4;

    let second = input
        .chars()
        .collect::<Vec<char>>()
        .windows(14)
        .position(|c| !(0..14).any(|i| (0..14).any(|y| c[i] == c[y] && y != i)))
        .unwrap()
        + 14;

    println!("{:?}", first);
    println!("{:?}", second);
}
