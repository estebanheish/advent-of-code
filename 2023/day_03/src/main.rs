use std::{
    collections::HashMap,
    io::{self, Read},
};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let grid: Vec<Vec<char>> = input.trim().lines().map(|l| l.chars().collect()).collect();
    let nrows = grid.len();
    let ncols = grid.first().unwrap().len();

    {
        let mut sum = 0;
        let mut symbol = false;
        let mut number = String::new();
        for row in 0..nrows {
            for col in 0..ncols {
                let c = grid[row][col];
                match c {
                    n if n.is_digit(10) => {
                        number.push(n);
                        if !symbol {
                            symbol = symbol_in_neighbours(&grid, (row, col), (nrows, ncols));
                        }
                    }
                    _ => {
                        if !number.is_empty() {
                            if symbol {
                                sum += number.parse::<usize>().unwrap();
                            }
                            symbol = false;
                            number = String::new();
                        }
                    }
                }
            }
            if !number.is_empty() {
                if symbol {
                    sum += number.parse::<usize>().unwrap();
                }
                symbol = false;
                number = String::new();
            }
        }

        println!("part 1 -> {sum}");
    }

    {
        let mut number = String::new();
        let mut actual_star = None;
        let mut gears: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
        for row in 0..nrows {
            if let Some(g_pos) = actual_star {
                let n: usize = number.parse().unwrap();
                gears
                    .entry(g_pos)
                    .and_modify(|v| v.push(n))
                    .or_insert(vec![n]);
            }
            actual_star = None;
            number = String::new();
            for col in 0..ncols {
                let c = grid[row][col];
                match c {
                    n if n.is_digit(10) => {
                        number.push(n);
                        if actual_star.is_none() {
                            actual_star = star_around(&grid, (row, col), (nrows, ncols));
                        }
                    }
                    _ => {
                        if let Some(g_pos) = actual_star {
                            let n: usize = number.parse().unwrap();
                            gears
                                .entry(g_pos)
                                .and_modify(|v| v.push(n))
                                .or_insert(vec![n]);
                        }
                        actual_star = None;
                        number = String::new();
                    }
                }
            }
        }

        println!(
            "part 2 -> {}",
            gears
                .iter()
                .map(|(_, v)| if v.len() == 2 { v[0] * v[1] } else { 0 })
                .sum::<usize>()
        );
    }
}

fn star_around(
    grid: &Vec<Vec<char>>,
    pos: (usize, usize),
    limit: (usize, usize),
) -> Option<(usize, usize)> {
    let ps: [(isize, isize); 8] = [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, 1),
        (-1, -1),
        (1, 1),
        (1, -1),
    ];

    for (y, x) in ps
        .iter()
        .map(|(y, x)| (pos.0 as isize + y, pos.1 as isize + x))
        .filter(|(y, x)| *y >= 0 && *x >= 0 && *y < (limit.0 as isize) && *x < (limit.1 as isize))
    {
        let c = grid[y as usize][x as usize];
        if c == '*' {
            return Some((y as usize, x as usize));
        }
    }

    None
}

fn symbol_in_neighbours(grid: &Vec<Vec<char>>, pos: (usize, usize), limit: (usize, usize)) -> bool {
    let ps: [(isize, isize); 8] = [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, 1),
        (-1, -1),
        (1, 1),
        (1, -1),
    ];

    ps.iter()
        .map(|(y, x)| (pos.0 as isize + y, pos.1 as isize + x))
        .filter(|(y, x)| *y >= 0 && *x >= 0 && *y < (limit.0 as isize) && *x < (limit.1 as isize))
        .any(|(y, x)| {
            let c = grid[y as usize][x as usize];
            c != '.' && !c.is_digit(10)
        })
}
