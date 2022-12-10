use std::collections::HashMap;

fn main() {
    let parsed = include_str!("../../input.txt")
        .lines()
        .map(|i| {
            let mut it = i.split(" ");
            let ins = it.next().unwrap();
            if ins == "noop" {
                return (ins, 0);
            }
            (ins, it.next().unwrap().parse().unwrap())
        })
        .collect::<Vec<(&str, isize)>>();

    let mut out: Vec<(isize, isize)> = vec![(0, 1)];
    for (ins, n) in parsed {
        let (cycle, register) = out.last().unwrap();
        if ins == "noop" {
            out.push((cycle + 1, *register));
            continue;
        } else {
            out.push((cycle + 2, register + n));
        }
    }

    let mut signal_strength: Vec<isize> = Vec::new();
    for n in [20, 60, 100, 140, 180, 220] {
        let mut tmp = 0;
        for (c, r) in &out {
            if *c >= n {
                break;
            }
            tmp = *r * n;
        }
        signal_strength.push(tmp);
    }

    println!("{}", signal_strength.iter().sum::<isize>()); // part 1

    let cpu: HashMap<isize, isize> = out.into_iter().collect();
    let mut tmp = cpu.get(&0).unwrap();
    for i in 1..=240 {
        if let Some(r) = cpu.get(&i) {
            tmp = r;
        }

        if [tmp - 1, *tmp, tmp + 1].contains(&(i % 40)) {
            print!("#");
        } else {
            print!(".");
        }

        if [40, 80, 120, 160, 200, 240].contains(&i) {
            print!("\n");
        }
    }
}
