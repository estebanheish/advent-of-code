#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
struct Point(usize, usize);

impl Point {
    fn ns(&self, lx: usize, ly: usize) -> Vec<Self> {
        let mut out = Vec::new();

        // up
        if self.0 > 0 {
            out.push(Point(self.0 - 1, self.1));
        }

        // down
        if self.0 < ly {
            out.push(Point(self.0 + 1, self.1));
        }

        // left
        if self.1 > 0 {
            out.push(Point(self.0, self.1 - 1));
        }

        // right
        if self.1 < lx {
            out.push(Point(self.0, self.1 + 1));
        }

        out
    }
}

fn main() {
    let input = include_bytes!("../../input.txt")
        .split(|x| x == &b'\n')
        .filter(|x| x != &[]);

    let mut paths: Vec<usize> = Vec::new();
    let grid: Vec<&[u8]> = input.collect();
    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            if grid[r][c] == b'S' || grid[r][c] == b'a' {
                if let Some(n) = sortest_path(Point(r, c), &grid) {
                    paths.push(n);
                }
            }
            if grid[r][c] == b'S' {
                println!("Part 1: {}", sortest_path(Point(r,c), &grid).unwrap());
            }
        }
    }

    println!("Part 2: {}", paths.iter().min().unwrap());
}

fn sortest_path(start: Point, input: &Vec<&[u8]>) -> Option<usize> {
    let ly = input.len() - 1;
    let lx = input[0].len() - 1;
    let mut visited: Vec<Point> = Vec::new();
    let mut to_visit: Vec<(Point, usize)> = vec![(start, 0)];
    while let Some((visiting @ Point(y, x), steps)) = to_visit.pop() {

        if input[y][x] == b'E' {
            return Some(steps);
        }

        let mut standing = input[y][x];
        if input[y][x] == b'S' {
            standing = b'a';
        }

        for p @ Point(a, b) in visiting.ns(lx, ly) {
            let n = if input[a][b] == b'E' {
                b'z'
            } else {
                input[a][b]
            };

            if standing + 1 >= n && !visited.contains(&p) && !to_visit.contains(&(p, steps + 1)) {
                to_visit.push((p, steps + 1));
            }
        }

        visited.push(visiting);
        to_visit.sort_by(|(_, s), (_, s2)| s2.cmp(s));
        // printvisited(&visited, lx, ly);
    }
    None
}

// fn printvisited(v: &Vec<Point>, yd: usize, xd: usize) {
//     std::process::Command::new("clear").status().unwrap();
//     for y in 0..=yd {
//         for x in 0..=xd {
//             let p = Point(y, x);
//             if v.contains(&p) {
//                 print!("#");
//             } else {
//                 print!(".");
//             }
//         }
//         print!("\n");
//     }
// }
