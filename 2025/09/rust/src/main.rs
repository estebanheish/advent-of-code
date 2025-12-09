use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let red_tiles: Vec<(i64, i64)> = input
        .trim()
        .lines()
        .map(|l| {
            let (a, b) = l.split_once(',').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect();

    let mut p1 = 0;
    let mut p2 = 0;
    for (i, a) in red_tiles.iter().enumerate() {
        for b in red_tiles.iter().skip(i + 1) {
            let area = (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1);
            p1 = p1.max(area);

            let lr = (a.0.min(b.0), a.0.max(b.0));
            let ud = (a.1.min(b.1), a.1.max(b.1));

            let mut i = false;
            for w in red_tiles.windows(2) {
                let [f, t] = w else {
                    continue;
                };

                if f.0 == t.0
                    && lr.0 < f.0
                    && lr.1 > f.0
                    && ud.0 < f.1.max(t.1)
                    && ud.1 > f.1.min(t.1)
                    || f.1 == t.1
                        && ud.0 < f.1
                        && ud.1 > f.1
                        && lr.0 < f.0.max(t.0)
                        && lr.1 > f.0.min(t.0)
                {
                    i = true;
                    break;
                }
            }

            if !i {
                p2 = p2.max(area);
            }
        }
    }

    println!("{p1}\n{p2}");
}
