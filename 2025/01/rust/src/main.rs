use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut p1 = 0;
    let mut p2 = 0;
    let mut now: i32 = 50;
    for line in input.lines() {
        let (d, n) = line.split_at(1);
        let n = n.parse::<i32>().unwrap() * if d == "L" { -1 } else { 1 };

        if n < 0 && now == 0 {
            p2 -= 1;
        }

        now += n;
        p2 += now.div_euclid(100).abs();
        now = now.rem_euclid(100);

        if n < 0 && now == 0 {
            p2 += 1;
        }

        if now == 0 {
            p1 += 1;
        }
    }

    println!("{p1}\n{p2}");
}
