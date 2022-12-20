fn decrypt(numbers: &mut Vec<(usize, isize)>) {
    let l = numbers.len();
    for i in 0..l {
        let p = numbers.iter().position(|(y, _)| *y == i).unwrap();
        let j @ (_, n) = numbers[p];
        if n != 0 {
            numbers.remove(p);
            let to = (p as isize + n as isize).rem_euclid(l as isize - 1);
            numbers.insert(to as usize, j);
        }
    }
}

fn coordinates(numbers: &Vec<(usize, isize)>) -> isize {
    let p_zero = numbers.iter().position(|(_, n)| *n == 0).unwrap();
    [1000, 2000, 3000]
        .iter()
        .map(|n| numbers[(*n + p_zero).rem_euclid(numbers.len())].1)
        .sum()
}

fn main() {
    let numbers: Vec<isize> = include_str!("../../input.txt")
        .lines()
        .map(|n| n.parse().unwrap())
        .collect();

    // part 1
    let mut part1: Vec<(usize, isize)> = numbers.clone().into_iter().enumerate().collect();
    decrypt(&mut part1);
    println!("part 1: {}", coordinates(&part1));

    // part 2
    let decryption_key = 811589153;
    let mut part2: Vec<(usize, isize)> = numbers.into_iter().map(|n| n * decryption_key).enumerate().collect();
    for _ in 0..10 {
        decrypt(&mut part2);
    }
    println!("part 2: {}", coordinates(&part2));
}
