use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
    io::{stdin, Read},
};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct State {
    row: usize,
    col: usize,
    cost: u32,
    last: Direction,
    kept: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let input = input.trim();

    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut heap = BinaryHeap::new();
    heap.push(State::def(Direction::Down));
    heap.push(State::def(Direction::Right));
    let mut seen = HashSet::new();
    while let Some(now) = heap.pop() {
        let a = (now.row, now.col, now.last, now.kept);
        if seen.contains(&a) {
            continue;
        }
        seen.insert(a);

        if now.row == grid.len() - 1 && now.col == grid.first().unwrap().len() - 1 {
            println!("part 1 -> {}", now.cost);
            break;
        }

        // up
        if now.last != Direction::Down
            && !(now.last == Direction::Up && now.kept == 3)
            && now.row > 0
        {
            heap.push(now.walk(Direction::Up, &grid))
        }

        // down
        if now.last != Direction::Up
            && !(now.last == Direction::Down && now.kept == 3)
            && now.row < grid.len() - 1
        {
            heap.push(now.walk(Direction::Down, &grid))
        }

        // left
        if now.last != Direction::Right
            && !(now.last == Direction::Left && now.kept == 3)
            && now.col > 0
        {
            heap.push(now.walk(Direction::Left, &grid))
        }

        // right
        if now.last != Direction::Left
            && !(now.last == Direction::Right && now.kept == 3)
            && now.col < grid[0].len() - 1
        {
            heap.push(now.walk(Direction::Right, &grid))
        }
    }

    let mut heap = BinaryHeap::new();
    heap.push(State::def(Direction::Down));
    heap.push(State::def(Direction::Right));
    let mut seen = HashSet::new();
    while let Some(now) = heap.pop() {
        let a = (now.row, now.col, now.last, now.kept);
        if seen.contains(&a) {
            continue;
        }
        seen.insert(a);

        if now.row == grid.len() - 1 && now.col == grid.first().unwrap().len() - 1 && now.kept >= 4
        {
            println!("part 2 -> {}", now.cost);
            break;
        }

        // up
        if now.last != Direction::Down
            && !(now.last == Direction::Up && now.kept == 10)
            && !(now.last != Direction::Up && now.kept < 4)
            && now.row > 0
        {
            heap.push(now.walk(Direction::Up, &grid))
        }

        // down
        if now.last != Direction::Up
            && !(now.last == Direction::Down && now.kept == 10)
            && !(now.last != Direction::Down && now.kept < 4)
            && now.row < grid.len() - 1
        {
            heap.push(now.walk(Direction::Down, &grid))
        }

        // left
        if now.last != Direction::Right
            && !(now.last == Direction::Left && now.kept == 10)
            && !(now.last != Direction::Left && now.kept < 4)
            && now.col > 0
        {
            heap.push(now.walk(Direction::Left, &grid))
        }

        // right
        if now.last != Direction::Left
            && !(now.last == Direction::Right && now.kept == 10)
            && !(now.last != Direction::Right && now.kept < 4)
            && now.col < grid[0].len() - 1
        {
            heap.push(now.walk(Direction::Right, &grid))
        }
    }
}

impl State {
    fn def(d: Direction) -> Self {
        State {
            row: 0,
            col: 0,
            cost: 0,
            last: d,
            kept: 0,
        }
    }
    fn walk(&self, d: Direction, g: &[Vec<u32>]) -> Self {
        let mut r = self.row;
        let mut c = self.col;
        match d {
            Direction::Up => r -= 1,
            Direction::Down => r += 1,
            Direction::Right => c += 1,
            Direction::Left => c -= 1,
        }
        Self {
            row: r,
            col: c,
            cost: self.cost + g[r][c],
            last: d,
            kept: if self.last == d { self.kept + 1 } else { 1 },
        }
    }
}
