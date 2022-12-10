use std::collections::HashSet;

#[derive(Default, Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Point(isize, isize);

fn main() {
    let motions: Vec<(char, usize)> = include_str!("../../input.txt")
        .lines()
        .map(|l| {
            let mut it = l.split(" ");
            (
                it.next().unwrap().chars().next().unwrap(),
                it.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    let mut knots: [Point; 10] = Default::default();
    let mut paths: [HashSet<Point>; 10] = Default::default();

    for (m, r) in motions {
        for _ in 0..r {
            knots[0].go(&m);
            for i in 1..10 {
                if knots[i].distance(knots[i - 1]) > 1 {
                    knots[i] = knots[i].follow(knots[i - 1]);
                }
                paths[i].insert(knots[i]);
            }
        }
    }

    println!("{}\n{}", paths[1].len(), paths[9].len());
}

impl Point {
    fn go(&mut self, c: &char) {
        match c {
            'U' => self.1 += 1,
            'D' => self.1 -= 1,
            'R' => self.0 += 1,
            'L' => self.0 -= 1,
            _ => unreachable!(),
        }
    }

    fn diff(&self, other: Self) -> Self {
        Point(other.0 - self.0, other.1 - self.1)
    }

    fn distance(&self, other: Self) -> isize {
        let Point(x, y) = self.diff(other);
        x.abs().max(y.abs())
    }

    fn follow(&self, other: Self) -> Self {
        let Point(x, y) = self.diff(other);
        let x = if x.abs() > 1 { x / 2 } else { x };
        let y = if y.abs() > 1 { y / 2 } else { y };
        Point(self.0 + x, self.1 + y)
    }
}
