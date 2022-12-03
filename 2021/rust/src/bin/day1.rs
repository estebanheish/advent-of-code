use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/day1.txt").unwrap();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(text: &str) -> usize {
    text.lines()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>()
        .windows(2)
        .filter(|x| -> bool {
            match x {
                [x, y] if y > x => true,
                _ => false,
            }
        })
        .count()
}

fn part2(text: &str) -> usize {
    text.lines()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>()
        .windows(4)
        .filter(|x| -> bool {
            match x {
                [f, s, t, c] if c > f => true,
                _ => false,
            }
        })
        .count()
}
