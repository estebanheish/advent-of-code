use std::collections::HashMap;
use std::collections::HashSet;

type Elfs = HashSet<Point>;
type Point = (isize, isize);

fn main() {
    let mut elfs: Elfs = HashSet::new();
    for (r, row) in include_str!("../../input.txt").lines().enumerate() {
        for (c, char) in row.chars().enumerate() {
            if char == '#' {
                elfs.insert((r as isize, c as isize));
            }
        }
    }

    let up = [(-1, -1), (-1, 0), (-1, 1)];
    let down = [(1, -1), (1, 0), (1, 1)];
    let right = [(1, 1), (0, 1), (-1, 1)];
    let left = [(-1, -1), (0, -1), (1, -1)];
    let moves = [up, down, left, right];
    let mut round: isize = 0;

    loop {
        let o = round.rem_euclid(4);
        round += 1;
        let mut proposals: HashMap<Point, Vec<Point>> = HashMap::new();
        let mut new_elfs: Elfs = HashSet::new();

        // proposals
        'elf: for elf @ (r, c) in &elfs {

            // if elf is alone
            if [(-1,-1),(-1,0),(-1,1),(0,1),(1,1),(1,0),(1,-1),(0,-1)]
            .iter()
            .all(|(y, x)| !elfs.contains(&(r + y, c + x)))
            {
                new_elfs.insert(*elf);
                continue;
            }
            for i in 0..4 {
                let im = (o + i).rem_euclid(4) as usize;
                if moves[im]
                    .iter()
                    .all(|(y, x)| !elfs.contains(&(*y + r, *x + c)))
                {
                    let p = (moves[im][1].0 + r, moves[im][1].1 + c);
                    proposals
                        .entry(p)
                        .and_modify(|v| v.push(*elf))
                        .or_insert(vec![*elf]);
                    continue 'elf;
                }
            }
            // elf is blocked
            new_elfs.insert(*elf);
        }

        // move
        for (proposed, elfs) in proposals {
            if elfs.len() > 1 {
                for elf in elfs {
                    new_elfs.insert(elf);
                }
            } else {
                new_elfs.insert(proposed);
            }
        }

        if elfs == new_elfs {
            println!("part 2: {}", round);
            break;
        }

        elfs = new_elfs;

        if round == 10 {
            let min_r = elfs.iter().map(|p| p.0).min().unwrap();
            let max_r = elfs.iter().map(|p| p.0).max().unwrap();
            let min_c = elfs.iter().map(|p| p.1).min().unwrap();
            let max_c = elfs.iter().map(|p| p.1).max().unwrap();
            let rectangle = ((max_r - min_r) + 1) * ((max_c - min_c) + 1);
            let empty = rectangle - elfs.len() as isize;
            println!("part 1: {}", empty);
        }
    }
}
