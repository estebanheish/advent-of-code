use std::collections::HashSet;
use std::collections::VecDeque;
use std::env;

type P = ((usize, usize), usize);

fn main() {
    let grid: Vec<&[u8]> = include_bytes!("../../input.txt")
        .split(|x| x == &b'\n')
        .filter(|x| x != &[])
        .collect();

    let points = grid
        .iter()
        .enumerate()
        .map(|(y, a)| {
            a.iter()
                .enumerate()
                .map(|(x, b)| ((y, x), *b))
                .collect::<Vec<((usize, usize), u8)>>()
        })
        .flatten();

    let args: Vec<String> = env::args().collect();
    let preview = args.contains(&"-p".to_string());

    let (start1, _): ((usize, usize), u8) = points.clone().find(|(_, b)| *b == b'S').unwrap();
    if let Some(part1) = sortest_path(preview, &grid, b'E', vec![start1]) {
        println!("part 1: {part1}");
    }

    let start2: Vec<(usize, usize)> = points.filter(|(_, b)| *b == b'a').map(|(p, _)| p).collect();
    if let Some(part2) = sortest_path(preview, &grid, b'E', start2) {
        println!("part 2: {part2}");
    }
}

fn sortest_path(
    pr: bool,
    grid: &Vec<&[u8]>,
    target: u8,
    start: Vec<(usize, usize)>,
) -> Option<usize> {
    let xl = (grid[0].len() - 1) as isize;
    let yl = (grid.len() - 1) as isize;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut to_visit: VecDeque<P> = VecDeque::new();
    for p in start {
        to_visit.push_front((p, 0));
    }

    while let Some(((y, x), steps)) = to_visit.pop_back() {
        let mut standing = grid[y][x];
        if standing == b'S' {
            standing = b'a';
        }

        for (y, x) in [
            (y as isize + 1, x as isize),
            (y as isize - 1, x as isize),
            (y as isize, x as isize + 1),
            (y as isize, x as isize - 1),
        ] {
            if y >= 0 && y <= yl && x >= 0 && x <= xl {
                let (y, x) = (y as usize, x as usize);
                let mut destination = grid[y][x];
                if destination == b'E' {
                    destination = b'z';
                }
                if standing + 1 >= destination && !visited.contains(&(y, x)) {
                    to_visit.push_front(((y, x), steps + 1));
                    visited.insert((y, x));

                    if target == grid[y][x] {
                        return Some(steps + 1);
                    }
                }
            }
        }
        visited.insert((y, x));
        if pr {
            printvisited(&visited, yl as usize, xl as usize);
        }
    }
    None
}

fn printvisited(v: &HashSet<(usize, usize)>, yd: usize, xd: usize) {
    std::process::Command::new("clear").status().unwrap();
    for y in 0..=yd {
        for x in 0..=xd {
            if v.contains(&(y, x)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}
