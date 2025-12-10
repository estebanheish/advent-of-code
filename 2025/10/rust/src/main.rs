use std::{
    collections::{HashSet, VecDeque},
    io::Read,
};

#[derive(Debug)]
struct Machine {
    lights: u64,
    buttons: Vec<u64>,
    _joltage: Vec<u64>,
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let machines = input
        .trim()
        .lines()
        .map(|l| {
            let s: Vec<_> = l.split(' ').collect();
            Machine {
                lights: s[0][1..s[0].len() - 1]
                    .chars()
                    .enumerate()
                    .fold(0, |acc, (i, c)| if c == '#' { acc | 1 << i } else { acc }),
                buttons: s[1..s.len() - 1]
                    .iter()
                    .map(|b| {
                        b[1..b.len() - 1]
                            .split(',')
                            .fold(0, |acc, d| acc | 1 << d.parse::<u64>().unwrap())
                    })
                    .collect(),
                _joltage: s[s.len() - 1][1..s[s.len() - 1].len() - 1]
                    .split(',')
                    .map(|d| d.parse().unwrap())
                    .collect(),
            }
        })
        .collect::<Vec<_>>();

    println!("{}", machines.iter().map(bfs_lights).sum::<u64>());
}

fn bfs_lights(m: &Machine) -> u64 {
    let mut queue = VecDeque::new();
    queue.push_back((0, 0));

    let mut seen = HashSet::new();
    while let Some((lights, buttons_pressed)) = queue.pop_front() {
        if seen.contains(&lights) {
            continue;
        }
        seen.insert(lights);
        if lights == m.lights {
            return buttons_pressed;
        }
        for button in &m.buttons {
            queue.push_back((lights ^ button, buttons_pressed + 1));
        }
    }

    0
}
