use std::collections::HashSet;

type Pair = ((isize, isize), (isize, isize));
type Point = (isize, isize);

fn md(ps: Pair) -> isize {
    let ((a, b), (c, d)) = ps;
    (a - c).abs() + (b - d).abs()
}

fn main() {
    let readings: Vec<Pair> = include_str!("../../input.txt")
        .lines()
        .map(|r| {
            let (s, b) = r.split_once(":").unwrap();
            let (sx, sy) = s.split_once(",").unwrap();
            let (bx, by) = b.split_once(",").unwrap();
            let f = |a: &str| -> isize {
                a.chars()
                    .filter(|c| c.is_digit(10) || *c == '-')
                    .collect::<String>()
                    .parse()
                    .unwrap()
            };
            ((f(sx), f(sy)), (f(bx), f(by)))
        })
        .collect();

    let max_x = readings.iter().map(|((x1, _), (x2, _))| x1.max(x2)).max().unwrap();
    let min_x = readings.iter().map(|((x1, _), (x2, _))| x1.min(x2)).min().unwrap();

    {
        // part 1
        let mut part1: HashSet<Point> = HashSet::new();
        let y = 2000000;
        for x in *min_x..*max_x {
            for r @ (s, b) in &readings {
                if md((*s, (x, y))) <= md(*r) && *b != (x, y) {
                    part1.insert((x, y));
                }
            }
        }
        println!("{}", part1.len());
    }

    {
        // part 2:
        let zz = 4000000;
        'outer: for r @ ((sx, sy), _) in &readings {
            let d = md(*r);
            let side = (0..=d + 1).zip((0..=d + 1).rev());
            let p = side.flat_map(|(x, y)| {
                [(1, 1), (-1, 1), (1, -1), (-1, -1)].map(|(a, b)| (x * a, y * b))
            });
            for (x, y) in p {
                let (x, y) = (x + sx, y + sy);
                if x >= 0 && x <= zz && y >= 0 && y <= zz {
                    let no = readings.iter().all(|d @ (s, _)| md((*s, (x, y))) > md(*d));
                    if no {
                        println!("{}", x * zz + y);
                        break 'outer;
                    }
                }
            }
        }
    }
}
