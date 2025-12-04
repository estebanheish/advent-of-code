use std::io::Read;

const NEIGHBOURS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let grid: Vec<Vec<char>> = input.trim().lines().map(|l| l.chars().collect()).collect();
    let cl = grid.len() as i32;
    let rl = grid.first().unwrap().len() as i32;

    let mut grid = grid;
    let mut n_removed = 0;
    loop {
        let mut to_remove = Vec::new();

        for r in 0..rl {
            for c in 0..cl {
                if grid[r as usize][c as usize] == '.' {
                    continue;
                }

                let npapers = NEIGHBOURS
                    .iter()
                    .map(|(a, b)| (r + a, c + b))
                    .filter(|(r, c)| {
                        !(*c < 0 || *r < 0 || *c >= cl || *r >= rl)
                            && grid[*r as usize][*c as usize] == '@'
                    })
                    .count();

                if npapers < 4 {
                    to_remove.push((r, c));
                }
            }
        }

        if n_removed == 0 {
            println!("{}", to_remove.len());
        }

        if to_remove.is_empty() {
            break;
        }

        n_removed += to_remove.len();

        for (r, c) in to_remove {
            grid[r as usize][c as usize] = '.';
        }
    }

    println!("{n_removed}");
}
