fn main() {
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let mut p: Vec<u32> = input
        .trim()
        .split("\n\n")
        .map(|elf| elf.split("\n").map(|n| n.parse::<u32>().unwrap()).sum())
        .collect();
    let l = p.len() - 1;
    p.sort();
    println!("{}", p[l]);
    println!("{}", p[l] + p[l - 1] + p[l - 2]);
}

