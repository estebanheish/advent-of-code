use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    input.pop();

    let mut p1: u64 = 0;
    let mut g: Vec<Vec<u64>> = Vec::new();
    for l in input.lines() {
        for (n_i, n) in l.split_ascii_whitespace().enumerate() {
            match n {
                "*" => p1 += g[n_i].iter().product::<u64>(),
                "+" => p1 += g[n_i].iter().sum::<u64>(),
                n => {
                    let n = n.parse().unwrap();
                    if let Some(l) = g.get_mut(n_i) {
                        l.push(n)
                    } else {
                        g.push(vec![n])
                    }
                }
            }
        }
    }

    let mut p2: u64 = 0;
    let g: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut tmp: Vec<u64> = Vec::new();
    for c in (0..g.first().unwrap().len()).rev() {
        let mut num = 0;
        for r in 0..g.len() {
            match g[r][c] {
                '*' => {
                    tmp.push(num);
                    p2 += tmp.iter().product::<u64>();
                    tmp.clear();
                }
                '+' => {
                    tmp.push(num);
                    p2 += tmp.iter().sum::<u64>();
                    tmp.clear();
                }
                ' ' if r == g.len() - 1 && num != 0 => tmp.push(num),
                ' ' => (),
                n => {
                    num *= 10;
                    num += (n as u8 - 48) as u64;
                }
            }
        }
    }

    println!("{p1}\n{p2}");
}
