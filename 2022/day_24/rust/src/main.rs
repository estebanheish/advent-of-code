use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

type Point = (isize, isize);
type Blizzards = HashMap<Point, Vec<char>>;

fn main() {
    let mut blizzards: Blizzards = Default::default();
    let mut max_y: isize = 0;
    let mut max_x: isize = 0;
    for (row, line) in include_str!("../../input.txt").lines().enumerate() {
        max_y = max_y.max(row as isize);
        for (col, c) in line.chars().enumerate() {
            max_x = max_x.max(col as isize);
            if c != '.' && c != '#' {
                blizzards.insert((row as isize, col as isize), vec![c]);
            }
        }
    }

    let moves: HashMap<char, Point> =
        [('^', (-1, 0)), ('v', (1, 0)), ('<', (0, -1)), ('>', (0, 1))].into();
    let mut blizzards_states = vec![blizzards.clone()];
    for _ in 0..max_x * max_y {
        let mut new_blizzards: Blizzards = HashMap::new();
        for ((y, x), cs) in blizzards_states.last().unwrap() {
            for c in cs {
                let m = moves.get(&c).unwrap();
                match (m.0 + y, m.1 + x) {
                    (y, x) if y == 0 => new_blizzards
                        .entry((max_y - 1, x))
                        .and_modify(|e| e.push(*c))
                        .or_insert(vec![*c]),
                    (y, x) if y == max_y => new_blizzards
                        .entry((1, x))
                        .and_modify(|e| e.push(*c))
                        .or_insert(vec![*c]),
                    (y, x) if x == 0 => new_blizzards
                        .entry((y, max_x - 1))
                        .and_modify(|e| e.push(*c))
                        .or_insert(vec![*c]),
                    (y, x) if x == max_x => new_blizzards
                        .entry((y, 1))
                        .and_modify(|e| e.push(*c))
                        .or_insert(vec![*c]),
                    _ => new_blizzards
                        .entry((y + m.0, x + m.1))
                        .and_modify(|e| e.push(*c))
                        .or_insert(vec![*c]),
                };
            }
        }
        blizzards_states.push(new_blizzards);
    }

    let bl = blizzards_states.len();

    let mut queue: VecDeque<(Point, usize, bool, bool)> = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back(((0, 1), 0, false, false));
    let mut first = false;
    while let Some(s @ (yo, min, mut t1, mut t2)) = queue.pop_front() {
        let bl_state = &blizzards_states[min.rem_euclid(bl)];
        if yo.0 == max_y && yo.1 == max_x - 1 && !first {
            println!("part 1: {}", min - 1);
            first = true;
        }
        if yo.0 == max_y && yo.1 == max_x - 1 && t1 && t2 {
            println!("part 2: {}", min - 1);
            break;
        }
        if yo.0 == max_y && yo.1 == max_x - 1 && !t1 {
            t1 = true;
        }
        if yo.0 == 0 && yo.1 == 1 && t1 && !t2 {
            t2 = true;
        }
        if visited.contains(&s) {
            continue;
        }
        visited.insert(s);

        if !bl_state.contains_key(&yo) {
            queue.push_back((yo, min + 1, t1, t2));
        }
        for m in moves.values() {
            let p @ (y, x) = (yo.0 + m.0, yo.1 + m.1);
            if !bl_state.contains_key(&p) {
                if y == 0 && x == 1 || y == max_y && x == max_x - 1 {
                    queue.push_back((p, min + 1, t1, t2));
                    continue;
                }
                if y > 0 && x > 0 && x < max_x && y < max_y {
                    queue.push_back((p, min + 1, t1, t2));
                }
            }
        }
    }
}
