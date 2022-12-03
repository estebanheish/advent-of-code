use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/day3.txt").unwrap();
    println!("{:?}", part1(&input));
    println!("{:?}", part2(&input));
}

fn part1(text: &str) -> usize {
    let txt = text.lines();
    let mut unos: [usize; 12] = [0; 12];
    for t in txt {
        for (i, c) in t.chars().enumerate() {
            match c {
                '1' => unos[i] += 1,
                _ => (),
            }
        }
    }
    let mut gamma = String::new();
    let mut epsilon = String::new();
    for i in &unos {
        if i >= &500 {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }
    usize::from_str_radix(&gamma, 2).unwrap() * usize::from_str_radix(&epsilon, 2).unwrap()
}

fn part2(text: &str) -> usize {
    let txt: Vec<&str> = text.lines().collect();
    let oxygen = nose(txt.clone(), 0, true);
    let co2 = nose(txt.clone(), 0, false);
    usize::from_str_radix(oxygen, 2).unwrap() * usize::from_str_radix(co2, 2).unwrap()
}

fn nose(b: Vec<&str>, c: usize, m: bool) -> &str {
    let l = b.len();
    if l == 1 {
        b[0]
    } else {
        let unos: usize = b
            .iter()
            .map(|x| x.chars().nth(c).unwrap().to_digit(10).unwrap() as usize)
            .sum();
        let common: usize = if unos >= l / 2 + (l % 2) {
            if m {
                1
            } else {
                0
            }
        } else {
            if m {
                0
            } else {
                1
            }
        };
        let filtered: Vec<&str> = b
            .iter()
            .filter(|x| common == x.chars().nth(c).unwrap().to_digit(10).unwrap() as usize)
            .map(|x| *x)
            .collect();
        nose(filtered, c + 1, m)
    }
}
