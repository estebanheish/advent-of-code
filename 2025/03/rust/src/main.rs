use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let input = input.trim();

    let mut p1 = 0;
    let mut p2 = 0;

    for line in input.lines() {
        let line: Vec<u64> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect();
        let l = line.len();

        let mut b = 0;
        for i in 0..l {
            for j in i + 1..l {
                b = b.max(line[i] * 10 + line[j]);
            }
        }
        p1 += b;

        let mut c = 0;
        for i in (0..12).rev() {
            let (idx, n) = line[c..l - i]
                .iter()
                .enumerate()
                .rev()
                .max_by_key(|(_, n)| *n)
                .unwrap();
            c += idx + 1;
            p2 += n * 10u64.pow(i as u32);
        }
    }

    println!("{p1}\n{p2}");
}
