use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let input: Vec<&str> = input.trim().split(',').collect();

    println!(
        "part 1 -> {}",
        input.iter().map(|s| hash(s.as_bytes())).sum::<usize>()
    );

    let mut boxes: Vec<Vec<(String, u32)>> = vec![vec![]; 256];
    for word in input {
        let mut it = word.chars();
        let label: String = it
            .by_ref()
            .take_while(|c| c.is_ascii_alphabetic())
            .collect();
        let hash = hash(label.as_bytes());
        if let Some(d) = it.next() {
            let focal_lens = d.to_digit(10).unwrap();
            if let Some(p) = boxes[hash].iter().position(|(l, _)| *l == label) {
                boxes[hash][p].1 = focal_lens;
            } else {
                boxes[hash].push((label, focal_lens));
            }
        } else if let Some(p) = boxes[hash].iter().position(|(l, _)| *l == label) {
            boxes[hash].remove(p);
        }
    }

    println!(
        "part 2 -> {}",
        boxes
            .iter()
            .enumerate()
            .map(|(i, v)| {
                v.iter()
                    .enumerate()
                    .map(|(j, (_, f))| (i + 1) * (j + 1) * *f as usize)
                    .sum::<usize>()
            })
            .sum::<usize>()
    );
}

fn hash(string: &[u8]) -> usize {
    string
        .iter()
        .fold(0, |acc, n| ((acc + *n as usize) * 17) % 256)
}
