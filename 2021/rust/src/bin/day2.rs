use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/day2.txt").unwrap();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(text: &str) -> usize {
    let mut pos = (0, 0);
    for m in text.lines() {
        match m.split_whitespace().collect::<Vec<_>>().as_slice() {
            ["forward", x] => pos.0 += x.parse::<usize>().unwrap(),
            ["down", x] => pos.1 += x.parse::<usize>().unwrap(),
            ["up", x] => pos.1 -= x.parse::<usize>().unwrap(),
            _ => (),
        }
    }
    pos.0 * pos.1
}

fn part2(text: &str) -> usize {
    let mut pos = (0, 0, 0); // hor, depth, aim
    for m in text.lines() {
        match m.split_whitespace().collect::<Vec<_>>().as_slice() {
            ["forward", x] => {
                pos.0 += x.parse::<usize>().unwrap();
                pos.1 += x.parse::<usize>().unwrap() * pos.2;
            }
            ["down", x] => pos.2 += x.parse::<usize>().unwrap(),
            ["up", x] => pos.2 -= x.parse::<usize>().unwrap(),
            _ => (),
        }
    }
    pos.0 * pos.1
}
