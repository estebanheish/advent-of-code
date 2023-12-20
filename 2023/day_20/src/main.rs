use std::{
    collections::{HashMap, VecDeque},
    io::{stdin, Read},
};

type Modules = HashMap<String, Module>;

#[derive(Debug, Clone)]
enum Module {
    Flipflop {
        state: bool,
        outputs: Vec<String>,
    },
    Conjunction {
        inputs: HashMap<String, bool>,
        outputs: Vec<String>,
    },
    Broadcaster {
        outputs: Vec<String>,
    },
}

impl Module {
    fn send_pulse(&mut self, is_high: bool, sender: &str) -> Option<(bool, &Vec<String>)> {
        match self {
            Module::Flipflop { state, outputs } if !is_high => {
                *state = !*state;
                Some((*state, outputs))
            }
            Module::Conjunction { inputs, outputs } => {
                *inputs.get_mut(sender).unwrap() = is_high;
                Some((inputs.values().any(|v| !*v), outputs))
            }
            Module::Broadcaster { outputs } => Some((is_high, outputs)),
            _ => None,
        }
    }

    fn conjorg(&mut self, origins: Vec<String>) {
        if let Module::Conjunction { inputs, .. } = self {
            for origin in origins {
                inputs.insert(origin, false);
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let input = input.trim();

    let mut modules: Modules = HashMap::new();
    let mut origins: HashMap<String, Vec<String>> = HashMap::new();
    input.lines().for_each(|l| {
        let (label, outputs) = l.split_once(" -> ").unwrap();
        let outputs: Vec<String> = outputs.split(", ").map(|s| s.to_string()).collect();
        if label == "broadcaster" {
            for o in &outputs {
                origins
                    .entry(o.to_owned())
                    .and_modify(|d| d.push(label.to_string()))
                    .or_insert(vec![label.to_string()]);
            }
            modules.insert(label.to_string(), Module::Broadcaster { outputs });
        } else if let Some(label) = label.strip_prefix('%') {
            for o in &outputs {
                origins
                    .entry(o.to_owned())
                    .and_modify(|d| d.push(label.to_string()))
                    .or_insert(vec![label.to_string()]);
            }
            modules.insert(
                label.to_string(),
                Module::Flipflop {
                    state: false,
                    outputs,
                },
            );
        } else {
            for o in &outputs {
                origins
                    .entry(o.to_owned())
                    .and_modify(|d| d.push(label[1..].to_string()))
                    .or_insert(vec![label[1..].to_string()]);
            }
            modules.insert(
                label[1..].to_string(),
                Module::Conjunction {
                    inputs: HashMap::new(),
                    outputs,
                },
            );
        }
    });

    for (d, os) in &origins {
        if d != "rx" {
            modules.get_mut(d).unwrap().conjorg(os.clone());
        }
    }

    println!("part 1 -> {}", part1(&mut modules.clone()));

    let last = origins.get("rx").unwrap().first().unwrap();
    let ll = origins.get(last).unwrap().len();

    println!(
        "part 2 -> lcm of {:?}",
        part2(&mut modules.clone(), last.to_owned(), ll)
    );
}

fn part1(modules: &mut Modules) -> usize {
    let mut low_pulses = 0;
    let mut high_pulses = 0;
    for _ in 0..1000 {
        let mut queue = VecDeque::new();

        low_pulses += 1;
        if let Module::Broadcaster { outputs } = modules.get("broadcaster").unwrap().clone() {
            for out in outputs {
                queue.push_back((false, out, "broadcaster".to_string()));
            }
        }

        while let Some((is_high, module, sender)) = queue.pop_front() {
            if is_high {
                high_pulses += 1;
            } else {
                low_pulses += 1;
            }
            if let Some(m) = modules.get_mut(&module) {
                if let Some((pulse, outputs)) = m.send_pulse(is_high, &sender) {
                    for output in outputs {
                        queue.push_back((pulse, output.to_owned(), module.clone()));
                    }
                }
            }
        }
    }

    low_pulses * high_pulses
}

fn part2(modules: &mut Modules, last: String, ll: usize) -> Vec<usize> {
    let mut presses = 0;
    let mut m_before = Vec::new();

    loop {
        presses += 1;
        let mut queue = VecDeque::new();
        if let Module::Broadcaster { outputs } = modules.get("broadcaster").unwrap().clone() {
            for out in outputs {
                queue.push_back((false, out, "broadcaster".to_string()));
            }
        }
        while let Some((is_high, module, sender)) = queue.pop_front() {
            if let Some(m) = modules.get_mut(&module) {
                if let Some((pulse, outputs)) = m.send_pulse(is_high, &sender) {
                    for output in outputs {
                        if &last == output && pulse {
                            m_before.push(presses);
                        }
                        queue.push_back((pulse, output.to_owned(), module.clone()));
                    }
                }
            }
        }

        if m_before.len() == ll {
            return m_before;
        }
    }
}
