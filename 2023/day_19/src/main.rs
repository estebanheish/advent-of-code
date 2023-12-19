use std::{
    collections::HashMap,
    io::{stdin, Read},
};

type Workflows = HashMap<String, Vec<Rule>>;

enum Rule {
    Test {
        c: char,
        n: usize,
        condition: char,
        destination: String,
    },
    Destination(String),
}

struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn sum(&self) -> usize {
        self.x + self.m + self.a + self.s
    }

    fn get(&self, c: char) -> usize {
        match c {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _ => unreachable!(),
        }
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let (workflows, parts) = input.trim().split_once("\n\n").unwrap();

    let parts: Vec<Part> = parts
        .lines()
        .map(|l| {
            let mut it = l[1..l.len() - 1].split(',');
            Part {
                x: it.next().unwrap()[2..].parse().unwrap(),
                m: it.next().unwrap()[2..].parse().unwrap(),
                a: it.next().unwrap()[2..].parse().unwrap(),
                s: it.next().unwrap()[2..].parse().unwrap(),
            }
        })
        .collect();

    let workflows: Workflows = workflows
        .lines()
        .map(|workflow| {
            let (label, rest) = workflow.split_once('{').unwrap();
            let rules: Vec<Rule> = rest[..rest.len() - 1]
                .split(',')
                .map(|rule| {
                    // dbg!(rule);
                    if let Some((condition, destination)) = rule.split_once(':') {
                        if let Some((c, n)) = condition.split_once('>') {
                            Rule::Test {
                                c: c.chars().next().unwrap(),
                                n: n.parse().unwrap(),
                                condition: '>',
                                destination: destination.to_string(),
                            }
                        } else {
                            let (c, n) = condition.split_once('<').unwrap();
                            Rule::Test {
                                c: c.chars().next().unwrap(),
                                n: n.parse().unwrap(),
                                condition: '<',
                                destination: destination.to_string(),
                            }
                        }
                    } else {
                        Rule::Destination(rule.to_string())
                    }
                })
                .collect();
            (label.to_string(), rules)
        })
        .collect();

    println!(
        "part 1 -> {}",
        parts
            .iter()
            .map(|part| {
                if apply_workflow(&workflows, part, "in".to_string()) {
                    part.sum()
                } else {
                    0
                }
            })
            .sum::<usize>()
    );

    println!(
        "part 2 -> {}",
        part2(&workflows, "in".to_string(), PartRange::new())
    );
}

#[derive(Copy, Clone)]
struct PartRange {
    x: (usize, usize),
    m: (usize, usize),
    a: (usize, usize),
    s: (usize, usize),
}

impl PartRange {
    fn new() -> Self {
        Self {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        }
    }

    fn cmb(&self) -> usize {
        (self.x.1 - self.x.0 + 1)
            * (self.m.1 - self.m.0 + 1)
            * (self.a.1 - self.a.0 + 1)
            * (self.s.1 - self.s.0 + 1)
    }

    fn apply_rule(&self, c: char, cnd: char, n: usize) -> (Self, Self) {
        match c {
            'x' if cnd == '>' => (
                Self {
                    x: (n + 1, self.x.1),
                    ..*self
                },
                Self {
                    x: (self.x.0, n),
                    ..*self
                },
            ),
            'x' if cnd == '<' => (
                Self {
                    x: (self.x.0, n - 1),
                    ..*self
                },
                Self {
                    x: (n, self.x.1),
                    ..*self
                },
            ),
            'm' if cnd == '>' => (
                Self {
                    m: (n + 1, self.m.1),
                    ..*self
                },
                Self {
                    m: (self.m.0, n),
                    ..*self
                },
            ),
            'm' if cnd == '<' => (
                Self {
                    m: (self.m.0, n - 1),
                    ..*self
                },
                Self {
                    m: (n, self.m.1),
                    ..*self
                },
            ),
            'a' if cnd == '>' => (
                Self {
                    a: (n + 1, self.a.1),
                    ..*self
                },
                Self {
                    a: (self.a.0, n),
                    ..*self
                },
            ),
            'a' if cnd == '<' => (
                Self {
                    a: (self.a.0, n - 1),
                    ..*self
                },
                Self {
                    a: (n, self.a.1),
                    ..*self
                },
            ),
            's' if cnd == '>' => (
                Self {
                    s: (n + 1, self.s.1),
                    ..*self
                },
                Self {
                    s: (self.s.0, n),
                    ..*self
                },
            ),
            's' if cnd == '<' => (
                Self {
                    s: (self.s.0, n - 1),
                    ..*self
                },
                Self {
                    s: (n, self.s.1),
                    ..*self
                },
            ),
            _ => unreachable!(),
        }
    }
}

fn part2(wls: &Workflows, now: String, range: PartRange) -> usize {
    if &now[..] == "A" {
        return range.cmb();
    }

    if &now[..] == "R" {
        return 0;
    }

    let mut out = 0;
    let mut range = range;
    for rule in wls.get(&now).unwrap() {
        match rule {
            Rule::Test {
                c,
                n,
                condition,
                destination,
            } => {
                let (valid, invalid) = range.apply_rule(*c, *condition, *n);
                range = invalid;
                out += part2(wls, destination.to_string(), valid);
            }
            Rule::Destination(d) => out += part2(wls, d.to_string(), range),
        }
    }
    out
}

fn apply_workflow(wfs: &Workflows, part: &Part, now: String) -> bool {
    match &now[..] {
        "A" => return true,
        "R" => return false,
        _ => (),
    }

    for wf in wfs.get(&now).unwrap() {
        match wf {
            Rule::Test {
                c,
                n,
                condition,
                destination,
            } => {
                let part_n = part.get(*c);
                if match condition {
                    '>' => part_n > *n,
                    '<' => part_n < *n,
                    _ => unreachable!(),
                } {
                    return apply_workflow(wfs, part, destination.to_owned());
                } else {
                    continue;
                }
            }
            Rule::Destination(d) => return apply_workflow(wfs, part, d.to_owned()),
        }
    }

    unreachable!();
}
