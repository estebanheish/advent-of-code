use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
struct Blueprint {
    ore: usize,
    clay: usize,
    obsidian: (usize, usize),
    geode: (usize, usize),
    max_ore: usize,
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct State {
    time_left: usize,
    //          ore, clay, obsidian, geode
    resources: (usize, usize, usize, usize),
    robots: (usize, usize, usize, usize),
}

impl State {
    fn new(time_left: usize) -> Self {
        State {
            time_left,
            resources: (0, 0, 0, 0),
            robots: (1, 0, 0, 0),
        }
    }
}

fn main() {
    let blueprints: Vec<Blueprint> = include_str!("../../input.txt")
        .lines()
        .map(|l| {
            let mut lit = l.split(".");
            let ore = lit
                .next()
                .unwrap()
                .split(" ")
                .nth(6)
                .unwrap()
                .parse()
                .unwrap();
            let clay = lit
                .next()
                .unwrap()
                .split(" ")
                .nth(5)
                .unwrap()
                .parse()
                .unwrap();

            let mut obsitidian_robot = lit.next().unwrap().split(" ");
            let obsidian_ore = obsitidian_robot.nth(5).unwrap().parse().unwrap();
            let obsidian_clay = obsitidian_robot.nth(2).unwrap().parse().unwrap();

            let mut geode_robot = lit.next().unwrap().split(" ");
            let geode_ore = geode_robot.nth(5).unwrap().parse().unwrap();
            let geode_obsidian = geode_robot.nth(2).unwrap().parse().unwrap();

            Blueprint {
                ore,
                clay,
                obsidian: (obsidian_ore, obsidian_clay),
                geode: (geode_ore, geode_obsidian),
                max_ore: *[ore, clay, obsidian_ore, geode_ore].iter().max().unwrap(),
            }
        })
        .collect();

    let mut part1 = 0;
    let mut part2 = 0;
    for (i, bp) in blueprints.iter().enumerate() {
        part1 += (i + 1) * geodes(*bp, 24);
        part2 += (i + 1) * geodes(*bp, 32);
    }

    println!("{}", part1);
    println!("{}", part2);
}

fn geodes(bp: Blueprint, time: usize) -> usize {
    let mut queue: Vec<State> = vec![State::new(time)];
    let mut geodes = 0;
    let mut cache: HashSet<State> = HashSet::new();

    while let Some(mut state) = queue.pop() {
        let (ore, clay, obsidian, geode) = state.resources;
        let (ore_robots, clay_robots, obsidian_robots, geode_robots) = state.robots;

        if state.time_left == 0 {
            geodes = geodes.max(geode);
            continue;
        }

        let t = state.time_left;
        if geode + geode_robots * t + ((t-1) * t / 2) <= geodes {
            continue;
        }

        if cache.contains(&state) {
            continue;
        }
        cache.insert(state);

        state.time_left -= 1;

        let new_resources = (
            ore + ore_robots,
            clay + clay_robots,
            obsidian + obsidian_robots,
            geode + geode_robots,
        );

        if ore >= bp.ore && ore_robots <= bp.max_ore {
            let mut state = state;
            state.robots.0 += 1;
            state.resources = new_resources;
            state.resources.0 -= bp.ore;
            queue.push(state);
        }

        if ore >= bp.clay && clay_robots <= bp.obsidian.1 {
            let mut state = state;
            state.robots.1 += 1;
            state.resources = new_resources;
            state.resources.0 -= bp.clay;
            queue.push(state);
        }

        if ore >= bp.obsidian.0 && clay >= bp.obsidian.1 && obsidian_robots <= bp.geode.1 {
            let mut state = state;
            state.robots.2 += 1;
            state.resources = new_resources;
            state.resources.0 -= bp.obsidian.0;
            state.resources.1 -= bp.obsidian.1;
            queue.push(state);
        }

        if ore >= bp.geode.0 && obsidian >= bp.geode.1 {
            let mut state = state;
            state.robots.3 += 1;
            state.resources = new_resources;
            state.resources.0 -= bp.geode.0;
            state.resources.2 -= bp.geode.1;
            queue.push(state);
        }

        state.resources = new_resources;
        queue.push(state);
    }
    geodes
}
