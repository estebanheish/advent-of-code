use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct Route {
    time_left: usize,
    position: String,
    opened: Vec<String>,
    released: usize,
}

impl Route {
    fn new(t: usize) -> Self {
        Route {
            time_left: t,
            position: "AA".to_string(),
            opened: Vec::new(),
            released: 0,
        }
    }
}

fn main() {
    let mut pipes: HashMap<String, (usize, Vec<String>)> = HashMap::new();
    let mut pipe_names = Vec::new();
    include_str!("../../input.txt").lines().for_each(|l| {
        let (name_and_flow_rate, lead) = l.split_once(";").unwrap();
        let (name, flow) = name_and_flow_rate.split_once("=").unwrap();
        let name: String = name.split(" ").skip(1).next().unwrap().to_string();
        let flow: usize = flow.parse().unwrap();
        let lead: Vec<String> = lead
            .split(",")
            .map(|w| w.chars().filter(|c| c.is_uppercase()).collect())
            .collect();
        pipes.insert(name.to_owned(), (flow, lead));
        pipe_names.push(name);
    });

    let mut steps: HashMap<(String, String), usize> = HashMap::new();
    {
        for pipe1 in &pipe_names {
            'f: for pipe2 in &pipe_names {
                let mut queue = VecDeque::new();
                queue.push_back((pipe1, 0));
                while let Some((pipe, step_count)) = queue.pop_front() {
                    let (_, ds) = pipes.get(pipe).unwrap();
                    for dp in ds {
                        if dp == pipe2 {
                            steps.insert((pipe1.to_owned(), pipe2.to_owned()), step_count + 1);
                            continue 'f;
                        }
                        queue.push_back((dp, step_count + 1));
                    }
                }
            }
        }
    }
    let pipe_names_no_zeros: Vec<String> = pipes
        .iter()
        .filter(|(_, (f, _))| *f > 0)
        .map(|(p, _)| p.to_owned())
        .collect();

    {
        // part 1
        let mut queue = Vec::new();
        queue.push(Route::new(30));
        let mut best = 0;
        while let Some(route) = queue.pop() {
            if route.time_left <= 0 {
                continue;
            }

            for pipe in &pipe_names_no_zeros {
                let mut route = route.clone();

                // skip already opened pipes
                if route.opened.contains(pipe) {
                    continue;
                }
                let steps = steps
                    .get(&(route.position.to_owned(), pipe.to_owned()))
                    .unwrap();

                // skip too far pipe
                if *steps >= route.time_left {
                    continue;
                }

                // open pipe
                let (flow_rate, _) = pipes.get(pipe).unwrap();
                route.position = pipe.to_string();
                route.time_left -= steps + 1;
                route.opened.push(pipe.to_owned());
                route.released += flow_rate * route.time_left;
                queue.push(route.clone()); // push opening too

                best = best.max(route.released);
            }
        }
        println!("part 1: {}", best);
    }

    {
        // part 2
        let mut queue2 = Vec::new();
        let mut queue = Vec::new();
        queue.push(Route::new(26));
        let mut best = 0;
        while let Some(route) = queue.pop() {
            if route.time_left <= 0 {
                let mut route = route;
                route.time_left = 26;
                route.position = "AA".to_string();
                queue2.push(route);
                continue;
            }

            for pipe in &pipe_names_no_zeros {
                let mut route = route.clone();

                // skip already opened pipes
                if route.opened.contains(pipe) {
                    continue;
                }
                let steps = steps
                    .get(&(route.position.to_owned(), pipe.to_owned()))
                    .unwrap();

                // skip too far pipe
                if *steps >= route.time_left {
                    continue;
                }

                // open pipe
                let (flow_rate, _) = pipes.get(pipe).unwrap();
                route.position = pipe.to_string();
                route.time_left -= steps + 1;
                route.opened.push(pipe.to_owned());
                route.released += flow_rate * route.time_left;
                queue.push(route.clone()); // push opening too

                best = best.max(route.released);
            }
        }

        let mut best = 0;
        while let Some(route) = queue2.pop() {
            if route.time_left <= 0 {
                continue;
            }

            for pipe in &pipe_names_no_zeros {
                let mut route = route.clone();

                // skip already opened pipes
                if route.opened.contains(pipe) {
                    continue;
                }
                let steps = steps
                    .get(&(route.position.to_owned(), pipe.to_owned()))
                    .unwrap();

                // skip too far pipe
                if *steps >= route.time_left {
                    continue;
                }

                // open pipe
                let (flow_rate, _) = pipes.get(pipe).unwrap();
                route.position = pipe.to_string();
                route.time_left -= steps + 1;
                route.opened.push(pipe.to_owned());
                route.released += flow_rate * route.time_left;
                queue2.push(route.clone()); // push opening too

                best = best.max(route.released);
            }
        }
        println!("part 2: {}", best);
    }
}
