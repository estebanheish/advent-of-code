use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
enum Op {
    Mul,
    Sum,
}

#[derive(Debug, Copy, Clone)]
enum OldOrNumber {
    Old,
    Number(usize),
}

#[derive(Debug, Copy, Clone)]
struct Operation {
    op: Op,
    first_number: OldOrNumber,
    second_number: OldOrNumber,
}

impl Operation {
    fn calc(&self, n: usize) -> usize {
        let f = match self.first_number {
            OldOrNumber::Number(i) => i,
            OldOrNumber::Old => n,
        };
        let s = match self.second_number {
            OldOrNumber::Number(i) => i,
            OldOrNumber::Old => n,
        };
        match self.op {
            Op::Mul => f * s,
            Op::Sum => f + s,
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    test: usize,           // divisible by
    throw: (usize, usize), // test true, false
    items_inspected: usize,
}

impl FromStr for OldOrNumber {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "old" {
            return Ok(Self::Old);
        } else {
            if let Ok(n) = s.parse::<usize>() {
                return Ok(Self::Number(n));
            } else {
                return Err(format!("Error parsing OldOrNumber {}", s));
            }
        }
    }
}

fn main() {
    let mut monkeys: Vec<Monkey> = include_str!("../../input.txt")
        .split("\n\n")
        .map(|m| {
            let mut monkey_it = m.split("\n").skip(1);
            let starting_items: Vec<usize> = monkey_it
                .next()
                .unwrap()
                .split(":")
                .skip(1)
                .next()
                .unwrap()
                .split(",")
                .map(|n| n.trim().parse().unwrap())
                .collect();

            let operation: Operation = {
                let mut op_it = monkey_it
                    .next()
                    .unwrap()
                    .split("=")
                    .skip(1)
                    .next()
                    .unwrap()
                    .trim()
                    .split(" ");
                let first_number: OldOrNumber = op_it.next().unwrap().parse().unwrap();
                let op = match op_it.next().unwrap() {
                    "+" => Op::Sum,
                    "*" => Op::Mul,
                    _ => unreachable!("bad input"),
                };
                let second_number: OldOrNumber = op_it.next().unwrap().parse().unwrap();
                Operation {
                    op,
                    first_number,
                    second_number,
                }
            };

            let test: usize = monkey_it
                .next()
                .unwrap()
                .split("by")
                .skip(1)
                .next()
                .unwrap()
                .trim()
                .parse()
                .unwrap();

            let throw: (usize, usize) = {
                let t: usize = monkey_it
                    .next()
                    .unwrap()
                    .split("monkey")
                    .skip(1)
                    .next()
                    .unwrap()
                    .trim()
                    .parse()
                    .unwrap();
                let f: usize = monkey_it
                    .next()
                    .unwrap()
                    .split("monkey")
                    .skip(1)
                    .next()
                    .unwrap()
                    .trim()
                    .parse()
                    .unwrap();
                (t, f)
            };

            Monkey {
                items: starting_items,
                operation,
                test,
                throw,
                items_inspected: 0,
            }
        })
        .collect();

    let mut monkeys2 = monkeys.clone();

    for _round in 0..20 {
        for i in 0..monkeys.len() {
            for nth_item in 0..monkeys[i].items.len() {
                let worried_level = monkeys[i].operation.calc(monkeys[i].items[nth_item]) / 3;
                monkeys[i].items_inspected += 1;
                if worried_level % monkeys[i].test == 0 {
                    let to = monkeys[i].throw.0;
                    monkeys[to].items.push(worried_level);
                } else {
                    let to = monkeys[i].throw.1;
                    monkeys[to].items.push(worried_level);
                }
            }
            monkeys[i].items = Vec::new();
        }
    }

    let mut items_inspected_per_monkey: Vec<usize> =
        monkeys.iter().map(|m| m.items_inspected).collect();
    items_inspected_per_monkey.sort_by(|a, b| b.cmp(a));
    println!(
        "{:?}",
        items_inspected_per_monkey.iter().take(2).product::<usize>()
    );

    let modulo: usize = monkeys2.iter().map(|m| m.test).product();
    for _round in 0..10000 {
        for i in 0..monkeys2.len() {
            for nth_item in 0..monkeys2[i].items.len() {
                let worried_level =
                    monkeys2[i].operation.calc(monkeys2[i].items[nth_item]) % modulo;
                monkeys2[i].items_inspected += 1;
                if worried_level % monkeys2[i].test == 0 {
                    let to = monkeys2[i].throw.0;
                    monkeys2[to].items.push(worried_level);
                } else {
                    let to = monkeys2[i].throw.1;
                    monkeys2[to].items.push(worried_level);
                }
            }
            monkeys2[i].items = Vec::new();
        }
    }

    let mut items_inspected_per_monkey: Vec<usize> =
        monkeys2.iter().map(|m| m.items_inspected).collect();
    items_inspected_per_monkey.sort_by(|a, b| b.cmp(a));
    println!(
        "{:?}",
        items_inspected_per_monkey.iter().take(2).product::<usize>()
    );
}
