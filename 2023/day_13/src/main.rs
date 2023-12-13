use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let input = input.trim();

    let grids: Vec<Vec<Vec<char>>> = input
        .split("\n\n")
        .map(|b| b.lines().map(|l| l.chars().collect()).collect())
        .collect();

    let mut part1 = 0;
    let mut part2 = 0;
    for grid in &grids {
        let transposed: Vec<Vec<_>> = (0..grid.first().unwrap().len())
            .map(|col| (0..grid.len()).map(|row| grid[row][col]).collect())
            .collect();

        part1 += reflectionl(&grid, 0) + reflectionl(&transposed, 0) * 100;
        part2 += reflectionl(&grid, 1) + reflectionl(&transposed, 1) * 100;
    }

    println!("part 1 -> {}", part1);
    println!("part 2 -> {}", part2);
}

fn is_mirrored(l: &[char]) -> usize {
    (0..l.len() / 2)
        .into_iter()
        .filter(|i| l[*i] != l[l.len() - 1 - i])
        .count()
}

fn reflectionl(ls: &Vec<Vec<char>>, f: usize) -> usize {
    let len = ls.first().unwrap().len();

    for i in (len % 2..len).step_by(2) {
        if ls.iter().map(|l| is_mirrored(&l[i..])).sum::<usize>() == f {
            return i + (len - i) / 2;
        }

        if ls.iter().map(|l| is_mirrored(&l[..len - i])).sum::<usize>() == f {
            return (len - i) / 2;
        }
    }
    0
}
