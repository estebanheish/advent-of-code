use std::collections::HashSet;

type JB = (i64, i64, i64);

fn distance(a: JB, b: JB) -> u64 {
    let x = a.0 - b.0;
    let y = a.1 - b.1;
    let z = a.2 - b.2;
    ((x.pow(2) + y.pow(2) + z.pow(2)) as f64).sqrt() as u64
}

fn merge_circuits(circuits: &[HashSet<JB>]) -> Vec<HashSet<JB>> {
    let mut merged = Vec::new();
    let mut done = vec![false; circuits.len()];

    for i in 0..circuits.len() {
        if done[i] {
            continue;
        }

        let mut current = circuits[i].clone();
        done[i] = true;

        loop {
            let mut changed = false;
            for j in 0..circuits.len() {
                if done[j] {
                    continue;
                }
                if !current.is_disjoint(&circuits[j]) {
                    current = current.union(&circuits[j]).copied().collect();
                    done[j] = true;
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }
        merged.push(current);
    }

    merged
}

fn main() {
    let mut boxes = Vec::new();
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            continue;
        }
        let mut it = line.split(',');
        let x: i64 = it.next().unwrap().parse().unwrap();
        let y: i64 = it.next().unwrap().parse().unwrap();
        let z: i64 = it.next().unwrap().parse().unwrap();
        boxes.push((x, y, z));
    }

    let mut distances = Vec::new();
    for i in 0..boxes.len() {
        for j in i + 1..boxes.len() {
            let a = boxes[i];
            let b = boxes[j];
            distances.push((distance(a, b), a, b));
        }
    }
    distances.sort();

    let mut circuits: Vec<HashSet<JB>> = Vec::new();
    'd: for (i, (_, a, b)) in distances.into_iter().enumerate() {
        if i == 1000 {
            circuits.sort_by_key(|b| std::cmp::Reverse(b.len()));
            println!(
                "{}",
                circuits
                    .iter()
                    .map(|h| h.len() as u64)
                    .take(3)
                    .product::<u64>()
            );
        }
        for c in &mut circuits {
            if c.contains(&a) || c.contains(&b) {
                c.insert(a);
                c.insert(b);
                if let Some(h) = circuits.first()
                    && h.len() == boxes.len()
                {
                    println!("{}", a.0 * b.0);
                    break 'd;
                }
                circuits = merge_circuits(&circuits);
                continue 'd;
            }
        }
        let mut h = HashSet::new();
        h.insert(a);
        h.insert(b);
        circuits.push(h);
    }
}
