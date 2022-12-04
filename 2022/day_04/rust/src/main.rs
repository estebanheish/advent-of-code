#[derive(Debug)]
struct Pair((u32, u32), (u32, u32));

impl Pair {
    fn inside(&self) -> bool {
        self.0 .0 >= self.1 .0 && self.0 .1 <= self.1 .1
            || self.1 .0 >= self.0 .0 && self.1 .1 <= self.0 .1
    }
    fn overlap(&self) -> bool {
        self.0 .1 >= self.1 .0 && self.1 .1 >= self.0 .0
    }
}

fn main() {
    let parsed: Vec<Pair> = include_str!("../../input.txt")
        .lines()
        .map(|l| {
            let (a, b) = l.split_once(",").unwrap();
            let (x, y) = a.split_once("-").unwrap();
            let (z, k) = b.split_once("-").unwrap();
            Pair(
                (x.parse().unwrap(), y.parse().unwrap()),
                (z.parse().unwrap(), k.parse().unwrap()),
            )
        })
        .collect();

    println!("{}", parsed.iter().filter(|pair| pair.inside()).count());
    println!("{}", parsed.iter().filter(|pair| pair.overlap()).count());
}
