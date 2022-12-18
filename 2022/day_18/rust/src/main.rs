fn main() {
    let scan: Vec<(isize, isize, isize)> = include_str!("../../input.txt")
        .lines()
        .map(|l| {
            let mut lit = l.split(",");
            let x = lit.next().unwrap().parse().unwrap();
            let y = lit.next().unwrap().parse().unwrap();
            let z = lit.next().unwrap().parse().unwrap();
            (x, y, z)
        })
        .collect();

    let moves = [
        (1, 0, 0),
        (-1, 0, 0),
        (0, 1, 0),
        (0, -1, 0),
        (0, 0, 1),
        (0, 0, -1),
    ];

    {
        let mut part1 = scan.len() * 6;
        for (x, y, z) in &scan {
            for (xm, ym, zm) in moves {
                if scan.contains(&(x + xm, y + ym, z + zm)) {
                    part1 -= 1;
                    continue;
                }
            }
        }
        println!("part 1: {:?}", part1);
    }

    {
        let mut part2 = 0;
        let (max_x, max_y, max_z) = scan.iter().fold((0, 0, 0), |(ax, ay, az), (x, y, z)| {
            (ax.max(*x), ay.max(*y), az.max(*z))
        });
        let (min_x, min_y, min_z) = scan.iter().fold((0, 0, 0), |(ax, ay, az), (x, y, z)| {
            (ax.min(*x), ay.min(*y), az.min(*z))
        });
        let mut queue = vec![(min_x - 1, min_y - 1, min_z - 1)];
        let mut visited = Vec::new();
        while let Some((x, y, z)) = queue.pop() {
            for p @ (x, y, z) in moves.iter().map(|(mx, my, mz)| (mx + x, my + y, mz + z)) {
                if !scan.contains(&p)
                    && !visited.contains(&p)
                    && x >= min_x - 1
                    && x <= max_x + 1
                    && y >= min_y - 1
                    && y <= max_y + 1
                    && z >= min_z - 1
                    && z <= max_z + 1
                {
                    queue.push(p);
                    visited.push(p);
                    for (mx, my, mz) in moves {
                        if scan.contains(&(mx + x, my + y, mz + z)) {
                            part2 += 1;
                        }
                    }
                }
            }
        }
        println!("part 2: {}", part2);
    }
}
