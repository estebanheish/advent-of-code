use std::{collections::HashMap, io::Read};

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let conexions: HashMap<&str, Vec<&str>> = input
        .trim()
        .lines()
        .map(|l| {
            let (input, outputs) = l.split_once(':').unwrap();
            (input, outputs.split_ascii_whitespace().collect())
        })
        .collect();

    println!(
        "{}\n{}",
        dfs_count("you", "out", &conexions, &mut HashMap::new()),
        dfs_count("svr", "fft", &conexions, &mut HashMap::new())
            * dfs_count("fft", "dac", &conexions, &mut HashMap::new())
            * dfs_count("dac", "out", &conexions, &mut HashMap::new())
    );
}

fn dfs_count<'a>(
    pos: &'a str,
    end: &str,
    conexions: &HashMap<&str, Vec<&'a str>>,
    cache: &mut HashMap<&'a str, u64>,
) -> u64 {
    if let Some(n) = cache.get(&pos) {
        return *n;
    }
    let mut n = 0;
    if pos == end {
        return n + 1;
    }
    if let Some(cons) = conexions.get(pos) {
        for c in cons {
            n += dfs_count(c, end, conexions, cache);
        }
    }
    cache.insert(pos, n);
    n
}
