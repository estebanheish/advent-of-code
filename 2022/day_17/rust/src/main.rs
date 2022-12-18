use std::collections::HashMap;
use std::collections::HashSet;

type Point = (usize, usize);
type Grid = HashSet<Point>;

fn highest_y(grid: &Grid) -> usize {
    if let Some(p) = grid.iter().map(|(_, y)| y).max() {
        return *p;
    } else {
        0
    }
}

fn main() {
    let jet_pattern: Vec<char> = include_str!("../../input.txt").trim().chars().collect();
    let jetl = jet_pattern.len();
    let mut grid: HashSet<Point> = HashSet::new();

    let line = vec![(0, 0), (1, 0), (2, 0), (3, 0)];
    let plus = vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)];
    let corner = vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)];
    let palo = vec![(0, 0), (0, 1), (0, 2), (0, 3)];
    let caja = vec![(0, 0), (1, 0), (0, 1), (1, 1)];
    let shapes = [line, plus, corner, palo, caja];

    let mut cur_shape: Vec<(usize, usize)> =
        shapes[0].iter().map(|(x, y)| (*x + 2, y + 3)).collect();
    let mut new_shapes = 0;
    let mut i = 0;
    let mut patterns_seen: HashMap<(Vec<Point>, usize, usize), (usize, usize)> = HashMap::new();
    let mut skipped = 0;

    let tu = 1_000_000_000_000;
    while new_shapes < tu {
        // move horizontally
        if jet_pattern[i % jetl] == '>' {
            // if nothing blocking right
            if !cur_shape
                .iter()
                .any(|(x, y)| *x == 6 || grid.contains(&(*x + 1, *y)))
            {
                // move right
                cur_shape.iter_mut().for_each(|(x, _)| *x += 1);
            }
        } else {
            // if nothing blocking left
            if !cur_shape
                .iter()
                .any(|(x, y)| *x == 0 || grid.contains(&(*x - 1, *y)))
            {
                // move left
                cur_shape.iter_mut().for_each(|(x, _)| *x -= 1);
            }
        }

        // move down
        // if shape blocked below
        if cur_shape
            .iter()
            .any(|(x, y)| *y == 0 || grid.contains(&(*x, y - 1)))
        {
            // add shape to grid
            cur_shape.iter().for_each(|p| {
                grid.insert(*p);
            });
            new_shapes += 1;
            let hy = highest_y(&grid);
            // create next shape
            cur_shape = shapes[new_shapes % 5]
                .iter()
                .map(|(x, y)| (*x + 2, y + hy + 4))
                .collect();

            if new_shapes == 2022 {
                println!("part 1: {}", highest_y(&grid) + 1);
            }

            if new_shapes > 2022 {
                let pattern: (Vec<Point>, usize, usize) = {
                    let mut p = grid
                        .iter()
                        .filter(|(_, y)| *y >= hy - 40)
                        .map(|(x, y)| (*x, hy - y))
                        .collect::<Vec<Point>>();
                    p.sort();
                    (p, i, new_shapes % 5)
                };

                if let Some((old_new_shapes, old_hy)) = patterns_seen.get(&pattern) {
                    let shapes_diff = new_shapes - old_new_shapes;
                    let height_diff = hy - old_hy;
                    let remaining = tu - new_shapes;
                    let factor = remaining / shapes_diff;
                    skipped += factor * height_diff;
                    new_shapes += factor * shapes_diff;
                } else {
                    patterns_seen.insert(pattern, (new_shapes, hy));
                }
            }
        } else {
            cur_shape.iter_mut().for_each(|(_, y)| *y -= 1);
        }

        i = (i + 1) % jetl;
    }
    println!("part 2: {}", highest_y(&grid) + 1 + skipped);
}
