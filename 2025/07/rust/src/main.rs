use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let g: Vec<Vec<char>> = input.trim().lines().map(|l| l.chars().collect()).collect();
    let w = g[0].len();
    let h = g.len();
    let s = g[0].iter().position(|&c| c == 'S').unwrap();

    let mut p1 = 0;

    let mut beams = vec![0; w];
    beams[s] = 1;
    for r in 0..h {
        let mut new = vec![0; w];
        for c in 0..w {
            if beams[c] > 0 {
                if g[r][c] == '^' {
                    p1 += 1;
                    if c > 0 {
                        new[c - 1] += beams[c];
                    }
                    if c < w - 1 {
                        new[c + 1] += beams[c];
                    }
                } else {
                    new[c] += beams[c];
                }
            }
        }
        beams = new;
    }

    let p2: u64 = beams.iter().sum();

    println!("{p1}\n{p2}");
}
