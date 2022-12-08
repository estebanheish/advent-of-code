#[derive(Copy, Clone)]
struct Point(usize, usize);

fn main() {
    let trees: Vec<Vec<u32>> = include_str!("../../input.txt")
        .lines()
        .map(|l| l.chars().map(|n| n.to_digit(10).unwrap()).collect())
        .collect();

    let mut v = 0;
    let mut s = 0;

    let l = trees.len();
    for r in 0..l {
        for c in 0..l {
            let btrees: Vec<Vec<bool>> = Point(r, c)
                .neighbours(l)
                .iter()
                .map(|line| {
                    line.iter()
                        .map(|Point(y, x)| trees[*y][*x] < trees[r][c])
                        .collect()
                })
                .collect();

            if btrees.iter().any(|l| l.iter().all(|j| *j == true)) {
                v += 1;
            }

            s = btrees
                .iter()
                .map(|l| {
                    if let Some(n) = l.iter().position(|b| *b == false) {
                        n + 1
                    } else {
                        l.len()
                    }
                })
                .product::<usize>()
                .max(s);
        }
    }
    println!("{}\n{}", v, s);
}

impl Point {
    fn up(&self) -> Vec<Self> {
        (0..self.0).rev().map(|n| Point(n, self.1)).collect()
    }
    fn down(&self, l: usize) -> Vec<Self> {
        (self.0 + 1..l).map(|n| Point(n, self.1)).collect()
    }
    fn left(&self) -> Vec<Self> {
        (0..self.1).rev().map(|n| Point(self.0, n)).collect()
    }
    fn right(&self, l: usize) -> Vec<Self> {
        (self.1 + 1..l).map(|n| Point(self.0, n)).collect()
    }
    fn neighbours(&self, l: usize) -> [Vec<Self>; 4] {
        [self.up(), self.down(l), self.left(), self.right(l)]
    }
}
