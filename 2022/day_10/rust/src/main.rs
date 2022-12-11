fn main() {
    let input = include_str!("../../input.txt");
    let mut out: Vec<isize> = vec![];
    let mut state = 1;
    for l in input.lines() {
        let mut lit = l.split(" ");
        let ins = lit.next().unwrap();
        if ins == "noop" {
            out.push(state);
        } else {
            let n: isize = lit.next().unwrap().parse().unwrap();
            out.push(state);
            out.push(state);
            state += n;
        }
    }
    let mut part1: isize = 0;
    for (c, r) in out.iter().enumerate() {
        if [19, 59, 99, 139, 179, 219].contains(&c) {
            part1 += (c as isize + 1) * r;
        }
        if [r - 1, *r, r + 1].contains(&(c as isize % 40)) {
            print!("#");
        } else {
            print!(".");
        }
        if [39, 79, 119, 159, 199, 239].contains(&c) {
            print!("\n");
        }
    }
    println!("{part1}");
}
