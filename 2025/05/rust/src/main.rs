use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let (ranges, ids) = input.split_once("\n\n").unwrap();
    let mut ranges: Vec<(u64, u64)> = ranges
        .lines()
        .map(|l| {
            let (a, b) = l.trim().split_once('-').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect();
    let ids: Vec<u64> = ids
        .trim()
        .lines()
        .map(|l| l.trim().parse().unwrap())
        .collect();

    let mut p1 = 0;
    'id: for id in ids {
        for (l, r) in &ranges {
            if id >= *l && id <= *r {
                p1 += 1;
                continue 'id;
            }
        }
    }

    let mut p2 = 0;
    ranges.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));

    let &(mut min, mut max) = ranges.first().unwrap();
    p2 += max - min + 1;
    for (l, r) in ranges.into_iter().skip(1) {
        p2 += r - l + 1;

        if l <= max {
            p2 -= max.min(r) - min.max(l) + 1;
        }

        min = l;
        max = max.max(r);
    }

    println!("{p1}\n{p2}");
}
