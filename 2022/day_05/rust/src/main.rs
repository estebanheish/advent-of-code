type Crates = Vec<Vec<char>>;
type Ins = Vec<(usize, usize, usize)>;

fn main() {
    let input = include_str!("../../input.txt");
    let mut crates: Vec<String> = input
        .lines()
        .take_while(|l| !l.chars().any(|c| c.is_numeric()))
        .map(|f| f.to_string())
        .collect();

    let ins: Ins = input
        .lines()
        .skip(crates.len() + 2)
        .map(|l| {
            let mut it = l.split(" ").skip(1).step_by(2).map(|n| n.parse().unwrap());
            (it.next().unwrap(), it.next().unwrap(), it.next().unwrap())
        })
        .collect();

    let crates = gen_crates(&mut crates);

    println!("{:?}", solve1(&crates, &ins));
    println!("{:?}", solve2(&crates, &ins));
}

fn solve2(crates: &Crates, ins: &Ins) -> String {
    let mut cs: Crates = crates.clone();

    for (m, f, t) in ins {
        let len = cs[*f - 1].len();
        let pile: Vec<char> = cs[*f - 1].drain(len - m..len).collect();
        for b in pile {
            cs[*t - 1].push(b);
        }
    }
    top_boxes(cs)
}

fn solve1(crates: &Crates, ins: &Ins) -> String {
    let mut cs: Crates = crates.clone();
    for (m, f, t) in ins {
        for _ in 0..*m {
            if let Some(b) = cs[f - 1].pop() {
                cs[t - 1].push(b);
            }
        }
    }
    top_boxes(cs)
}

fn top_boxes(crates: Crates) -> String {
    let mut out = String::new();
    for pile in crates {
        if let Some(b) = pile.last() {
            out.push(*b);
        }
    }
    out
}

fn gen_crates(cs: &mut Vec<String>) -> Crates {
    let mut out: Crates = (0..10).map(|_| Vec::new()).collect();
    cs.reverse();
    for row in cs {
        for (i, col) in (1..row.len()).step_by(4).enumerate() {
            if let Some(char) = row.chars().nth(col) {
                if char != ' ' {
                    out[i].push(char);
                }
            }
        }
    }
    out
}
