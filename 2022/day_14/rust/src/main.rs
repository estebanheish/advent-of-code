use core::iter::repeat;
use std::cmp::Ordering::{Equal, Greater, Less};

type P = (usize, usize);

#[derive(Clone, Copy)]
enum Tile {
    Air,
    Rock,
    Sand,
    Origin,
}

struct Cave {
    points: Vec<Vec<Tile>>,
    y: usize,
}

impl Cave {
    fn empty(y: usize, x: usize) -> Self {
        let mut points: Vec<Vec<Tile>> = Vec::with_capacity(y);
        let mut row: Vec<Tile> = Vec::with_capacity(x);
        for _ in 0..=x {
            row.push(Tile::Air);
        }
        for _ in 0..=y {
            points.push(row.clone());
        }
        Cave { points, y }
    }

    fn show(&self) {
        for row in &self.points {
            for tile in row {
                match tile {
                    Tile::Air => print!("."),
                    Tile::Rock => print!("#"),
                    Tile::Sand => print!("o"),
                    Tile::Origin => print!("+"),
                }
            }
            print!("\n");
        }
    }

    fn drop_sand(&self, p: P) -> Option<P> {
        let (mut y, mut x) = p;
        loop {
            if y > self.y {
                return None;
            }
            match self.points[y][x] {
                Tile::Air => (),
                Tile::Origin => (),
                _ => match self.points[y][x - 1] {
                    Tile::Origin => {
                        x -= 1;
                        continue;
                    }
                    Tile::Air => {
                        x -= 1;
                        continue;
                    }
                    _ => match self.points[y][x + 1] {
                        Tile::Air => {
                            x += 1;
                            continue;
                        }
                        Tile::Origin => {
                            x += 1;
                            continue;
                        }
                        Tile::Rock => return Some((y - 1, x)),
                        Tile::Sand => return Some((y - 1, x)),
                    },
                },
            }

            y += 1;
        }
    }

}

fn range(x: (usize, usize), y: (usize, usize)) -> Vec<(usize, usize)> {
    let (ax, ay) = x;
    let (bx, by) = y;

    match (ay.cmp(&by), ax.cmp(&bx)) {
        (Less, Equal) => (ay..=by).zip(repeat(ax)).collect(),
        (Greater, Equal) => (by..=ay).zip(repeat(ax)).collect(),
        (Equal, Less) => repeat(ay).take(bx - ax + 1).zip(ax..=bx).collect(),
        (Equal, Greater) => repeat(ay).take(ax - bx + 1).zip(bx..=ax).collect(),
        (Equal, Equal) => (ay..=by).zip(repeat(ax)).collect(),
        (Less, Greater) => unreachable!(),
        (Less, Less) => unreachable!(),
        (Greater, Less) => unreachable!(),
        (Greater, Greater) => unreachable!(),
    }
}

fn main() {
    let scan: Vec<Vec<(usize, usize)>> = include_str!("../../input.txt")
        .lines()
        .map(|l| {
            l.split(" -> ")
                .map(|p| {
                    let (a, b) = p.split_once(",").unwrap();
                    (a.parse().unwrap(), b.parse().unwrap())
                })
                .collect()
        })
        .collect();

    let mut max_y = 0;
    let mut max_x = 0;
    let mut min_x = usize::MAX;
    for row in &scan {
        for (a, b) in row {
            max_x = max_x.max(*a);
            max_y = max_y.max(*b);
            min_x = min_x.min(*a);
        }
    }

    let mut cave = Cave::empty(max_y+2, max_x + 500);

    cave.points[0][500] = Tile::Origin;

    for row in scan {
        let mut row = row.iter();
        let mut p1 = row.next().unwrap();
        while let Some(p2) = row.next() {
            for (y, x) in range(*p1, *p2) {
                cave.points[y][x] = Tile::Rock;
            }
            p1 = p2;
        }
    }

    let mut part1 = 0;
    loop {
        if let Some((y, x)) = cave.drop_sand((0, 500)) {
            cave.points[y][x] = Tile::Sand;
        } else {
            break;
        }
        part1 += 1;
    }

    println!("{}", part1);

    for x in 0..max_x+500 {
        cave.points[max_y+2][x] = Tile::Rock;
    }

    let mut part2 = 0;
    while let Tile::Origin = cave.points[0][500] {
        if let Some((y, x)) = cave.drop_sand((0, 500)) {
            cave.points[y][x] = Tile::Sand;
        } else {
            break;
        }
        part2 += 1;
    }

    println!("{}", part2+part1);
}
