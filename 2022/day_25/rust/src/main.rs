#![feature(int_roundings)]

fn to_snafu(n: isize, s: String) -> String {
    if n == 0 {
        return s;
    }
    let nn = n.div_floor(5);
    let rem = n.rem_euclid(5);
    match rem {
        3 => to_snafu(nn + 1, format!("={}", s)),
        4 => to_snafu(nn + 1, format!("-{}", s)),
        _ => to_snafu(nn, format!("{}{}", rem, s)),
    }
}

fn main() {
    let sum = include_str!("../../input.txt")
        .lines()
        .map(|l| {
            l.chars()
                .rev()
                .enumerate()
                .map(|(i, c)| -> isize {
                    let f: isize = match c {
                        '=' => -2isize,
                        '-' => -1isize,
                        _ => c.to_digit(10).unwrap() as isize,
                    };
                    (5usize.pow(i as u32) as isize * f) as isize
                })
                .sum::<isize>()
        })
        .sum::<isize>();

    println!("part 1: {}", to_snafu(sum, String::new()));
}
