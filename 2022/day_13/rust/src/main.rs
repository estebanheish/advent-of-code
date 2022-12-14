use std::cmp::Ordering;
use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Packet {
    N(usize),
    L(Vec<Packet>),
}

impl FromStr for Packet {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('[') {
            let l = s.chars().count();
            let s: String = s.chars().skip(1).take(l - 2).collect();
            let comas: Vec<usize> = {
                let mut out: Vec<usize> = Vec::new();
                let mut b = 0;
                for (i, c) in s.chars().enumerate() {
                    if c == '[' {
                        b += 1;
                    }
                    if c == ']' {
                        b -= 1;
                    }

                    if c == ',' && b == 0 {
                        out.push(i);
                    }
                }
                out
            };
            let mut out: Vec<Packet> = Vec::new();
            let mut tmp = String::new();
            for (i, c) in s.chars().enumerate() {
                if comas.contains(&i) {
                    out.push(tmp.parse::<Packet>()?);
                    tmp = String::new();
                    continue;
                }
                tmp.push(c);
            }
            if tmp != "" {
                out.push(tmp.parse::<Packet>()?);
            }
            return Ok(Packet::L(out));
        } else {
            if let Ok(n) = s.parse::<usize>() {
                return Ok(Packet::N(n));
            } else {
                return Ok(Packet::L(Vec::new()));
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            o @ Packet::N(n1) => match other {
                Packet::N(n2) => n1.cmp(n2),
                i @ Packet::L(_l2) => Packet::L(vec![o.clone()]).cmp(i),
            },
            o @ Packet::L(l1) => match other {
                i @ Packet::N(_n2) => o.cmp(&Packet::L(vec![i.clone()])),
                Packet::L(l2) => {
                    if l1.is_empty() && l2.is_empty() {
                        return Ordering::Equal;
                    }
                    let mut l1 = l1.iter();
                    let mut l2 = l2.iter();
                    loop {
                        if let Some(p1) = l1.next() {
                            if let Some(p2) = l2.next() {
                                match p1.cmp(p2) {
                                    o @ Ordering::Less => return o,
                                    o @ Ordering::Greater => return o,
                                    Ordering::Equal => continue,
                                }
                            } else {
                                return Ordering::Greater;
                            }
                        } else {
                            return Ordering::Less;
                        }
                    }
                }
            },
        }
    }
}

fn main() {
    let packets: Vec<(Packet, Packet)> = include_str!("../../input.txt")
        .split("\n\n")
        .map(|pair| {
            let mut pair_it = pair.lines();
            let a = pair_it.next().unwrap().parse().unwrap();
            let b = pair_it.next().unwrap().parse().unwrap();
            (a, b)
        })
        .collect();

    let mut part1 = 0;
    for (i, (a, b)) in packets.iter().enumerate() {
        match a.cmp(b) {
            Ordering::Less => {
                part1 += i + 1;
            }
            Ordering::Greater => continue,
            Ordering::Equal => unreachable!(),
        }
    }
    println!("{}", part1);

    // part 2
    let mut packets: Vec<Packet> = packets
        .clone()
        .iter()
        .map(|(a, b)| vec![a.clone(), b.clone()])
        .flatten()
        .collect();

    let delimiter1 = "[[2]]".parse::<Packet>().unwrap();
    let delimiter2 = "[[6]]".parse::<Packet>().unwrap();
    packets.push(delimiter1.clone());
    packets.push(delimiter2.clone());
    packets.sort();

    let part2: usize = packets
        .iter()
        .enumerate()
        .filter(|(_, x)| **x == delimiter1 || **x == delimiter2)
        .map(|(i, _)| i+1)
        .product();

    println!("{}", part2);
}
