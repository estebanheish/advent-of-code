use std::{
    collections::HashSet,
    io::{stdin, Read},
};

type Grid = Vec<Vec<char>>;

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let input = input.trim();

    let grid: Grid = input.lines().map(|l| l.chars().collect()).collect();

    println!("part 1 -> {}", explore(&grid, 64));

    let lgrid = grid.len();
    let steps = 26501365;
    let rem = steps % lgrid;
    let e1 = explore(&grid, rem);
    let e2 = explore(&grid, rem + lgrid);
    let e3 = explore(&grid, rem + lgrid * 2);
    let a = (e1 - 2 * e2 + e3) / 2;
    let b = e2 - e1 - a;
    let c = e1;
    let s = (steps / lgrid) as isize;
    println!("part 2 -> {}", a * s.pow(2) + b * s + c);
}

fn explore(grid: &Grid, n: usize) -> isize {
    let mut queue = Vec::new();

    'outer: for (row, l) in grid.iter().enumerate() {
        for (col, char) in l.iter().enumerate() {
            if *char == 'S' {
                queue.push((row as isize, col as isize));
                break 'outer;
            }
        }
    }

    let max_rows = grid.len() as isize;
    let max_cols = grid.first().unwrap().len() as isize;

    let mr = |r: isize| r.rem_euclid(max_rows) as usize;
    let mc = |c: isize| c.rem_euclid(max_cols) as usize;

    for _ in 0..n {
        let mut nexts = HashSet::new();
        while let Some((row, col)) = queue.pop() {
            {
                let row = row - 1;
                if grid[mr(row)][mc(col)] != '#' {
                    nexts.insert((row, col));
                }
            }

            {
                let row = row + 1;
                if grid[mr(row)][mc(col)] != '#' {
                    nexts.insert((row, col));
                }
            }

            {
                let col = col + 1;
                if grid[mr(row)][mc(col)] != '#' {
                    nexts.insert((row, col));
                }
            }

            {
                let col = col - 1;
                if grid[mr(row)][mc(col)] != '#' {
                    nexts.insert((row, col));
                }
            }
        }
        queue = nexts.into_iter().collect();
    }
    queue.len() as isize
}
