use std::collections::HashMap;

type Point = (isize, isize);
type Maze = HashMap<Point, char>;

#[derive(Debug)]
enum Ins {
    N(usize),
    R,
    L,
}

fn y_bounds(m: &Maze, x: isize) -> (isize, isize) {
    let ys = m.keys().filter(|(_, xx)| *xx == x).map(|(y, _)| y);
    (*ys.clone().min().unwrap(), *ys.max().unwrap())
}

fn x_bounds(m: &Maze, y: isize) -> (isize, isize) {
    let xs = m.keys().filter(|(yy, _)| *yy == y).map(|(_, x)| x);
    (*xs.clone().min().unwrap(), *xs.max().unwrap())
}

fn main() {
    let (maze_raw, path_raw) = include_str!("../../input.txt").split_once("\n\n").unwrap();
    let mut path_it = path_raw.trim().chars();
    let mut path: Vec<Ins> = Vec::new();

    let mut number: String = String::new();
    while let Some(c) = path_it.next() {
        if c.is_digit(10) {
            number.push(c);
        } else {
            path.push(Ins::N(number.parse().unwrap()));
            number = String::new();
            if c == 'L' {
                path.push(Ins::L);
            }
            if c == 'R' {
                path.push(Ins::R);
            }
        }
    }
    if number.len() > 0 {
        path.push(Ins::N(number.parse().unwrap()));
    }
    let maze_raw: Vec<Vec<char>> = maze_raw.lines().map(|l| l.chars().collect()).collect();
    let max_row = maze_raw.len();
    let max_col = maze_raw.iter().map(|l| l.len()).max().unwrap();

    // build maze
    let mut maze: Maze = HashMap::new();
    for row in 0..max_row {
        for col in 0..max_col {
            if let Some(line) = maze_raw.get(row) {
                if let Some(c) = line.get(col) {
                    if *c != ' ' {
                        maze.insert((row as isize, col as isize), *c);
                    }
                }
            }
        }
    }

    {
        // part 1
        // 0 for right (>), 1 for down (v), 2 for left (<), and 3 for up (^)
        let mut facing: isize = 0;
        let mut pos: Point = (0, x_bounds(&maze, 0).0);
        for ins in &path {
            match ins {
                Ins::R => facing = (facing + 1).rem_euclid(4),
                Ins::L => facing = (facing - 1).rem_euclid(4),
                Ins::N(steps) => {
                    for _ in 0..*steps {
                        if let Some(p) = step(pos, &maze, facing) {
                            pos = p;
                        } else {
                            break;
                        }
                    }
                }
            }
        }
        let part1 = (pos.0 + 1) * 1000 + (pos.1 + 1) * 4 + facing;
        println!("part 1: {}", part1);
    }

    {
        // part 2
        let mut facing: isize = 0;
        let mut pos: Point = (0, x_bounds(&maze, 0).0);
        for ins in path {
            match ins {
                Ins::R => facing = (facing + 1).rem_euclid(4),
                Ins::L => facing = (facing - 1).rem_euclid(4),
                Ins::N(steps) => {
                    for _ in 0..steps {
                        if let Some((p, f)) = step2(pos, &maze, facing) {
                            pos = p;
                            facing = f;
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        let part2 = (pos.0 + 1) * 1000 + (pos.1 + 1) * 4 + facing;
        println!("part 2: {}", part2);
    }
}

fn step(p: Point, maze: &Maze, f: isize) -> Option<Point> {
    let (a, b): (isize, isize) = [(0, 1), (1, 0), (0, -1), (-1, 0)][f as usize];
    let (mut y, mut x) = (p.0 + a, p.1 + b);

    if f == 0 || f == 2 {
        let (min_x, max_x) = x_bounds(&maze, y);
        if x < min_x {
            x = max_x;
        }
        if x > max_x {
            x = min_x;
        }
    } else {
        let (min_y, max_y) = y_bounds(&maze, x);
        if y < min_y {
            y = max_y;
        }
        if y > max_y {
            y = min_y;
        }
    }

    if let Some(c) = maze.get(&(y, x)) {
        if *c != '#' {
            return Some((y, x));
        } else {
            return None;
        }
    }
    unreachable!();
}

fn step2(p: Point, maze: &Maze, f: isize) -> Option<(Point, isize)> {
    assert!([0, 1, 2, 3].contains(&f), "{}", f);
    let (a, b): (isize, isize) = [(0, 1), (1, 0), (0, -1), (-1, 0)][f as usize];
    let (mut y, mut x) = (p.0 + a, p.1 + b);
    let mut f = f;

    match (y,x) {
        (r,c) if r < 0    && c >= 50  && c < 100 => { y = c + 100; x = 0; f = 0; },
        (r,c) if r < 0    && c >= 100 && c < 150 => { y = 199; x = c - 100; },
        (r,c) if c < 0    && r >= 150 && r < 200 => { y = 0; x = r - 100; f = 1; },
        (r,c) if c < 0    && r >= 100 && x < 150 => { y = 149 - r; x = 50; f = 0; },
        (r,c) if c == 49  && r >= 50  && r < 100 => { y = 100; x = r - 50; f = 1; },
        (r,c) if c == 49  && r >= 0   && r < 50 => { y = 149 - r; x = 0; f = 0; },
        (r,c) if c == 50  && r >= 150 && r < 200 => { y = 149; x = r - 100; f = 3; }, 
        (r,c) if r == 50  && c >= 100 && c < 150 => { y = c - 50 ; x = 99; f = 2; },
        (r,c) if r == 99  && c >= 0   && c < 50 => { y = c + 50; x = 50; f = 0; },
        (r,c) if c == 100 && r >= 100 && r < 150 => { y = 149 - r; x = 149; f = 2; },
        (r,c) if c == 100 && r >= 50  && r < 100 => { y = 49; x = r + 50; f = 3; },
        (r,c) if r == 150 && c >= 50  && c < 100 => { y = c + 100; x = 49; f = 2; },
        (r,c) if c >= 150 && r >= 0   && r < 50 => { y = 149 - r; x = 99; f = 2; },
        (r,c) if r >= 200 && c >= 0   && c < 50 => { y = 0; x = c + 100; },
        _ => (),
    }

    if let Some(c) = maze.get(&(y, x)) {
        if *c != '#' {
            return Some(((y, x), f));
        } else {
            return None;
        }
    }
    unreachable!();
}
