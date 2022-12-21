use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
enum Monkey<'a> {
    Num(f64),
    Expr(&'a str, Op, &'a str),
}

#[derive(Debug, Clone)]
enum Monkeys {
    Num(f64),
    Expr(Box<Monkeys>, Op, Box<Monkeys>),
}

#[derive(Debug, Copy, Clone)]
enum Op {
    Sum,
    Sub,
    Mul,
    Div,
}

impl FromStr for Op {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Op::Sum),
            "-" => Ok(Op::Sub),
            "*" => Ok(Op::Mul),
            "/" => Ok(Op::Div),
            _ => Err(format!("Error parsing operator {s}")),
        }
    }
}

fn build_monkeys(r: &str, m: &HashMap<&str, Monkey>, p2: bool, init: f64) -> Monkeys {
    if p2 && r == "humn" {
        return Monkeys::Num(init);
    }
    let r = m.get(r).unwrap();
    match r {
        Monkey::Num(n) => Monkeys::Num(*n),
        Monkey::Expr(name1, op, name2) => Monkeys::Expr(
            Box::new(build_monkeys(name1, m, p2, init)),
            *op,
            Box::new(build_monkeys(name2, m, p2, init)),
        ),
    }
}

fn eval(m: Monkeys) -> f64 {
    match m {
        Monkeys::Num(n) => n,
        Monkeys::Expr(n1, op, n2) => {
            let n1 = eval(*n1);
            let n2 = eval(*n2);
            match op {
                Op::Sum => n1 + n2,
                Op::Sub => n1 - n2,
                Op::Mul => n1 * n2,
                Op::Div => n1 / n2,
            }
        }
    }
}

fn main() {
    let input = include_str!("../../input.txt");
    let mut monkeys: HashMap<&str, Monkey> = HashMap::new();
    input.lines().for_each(|l| {
        let mut lit = l.split(" ");
        let name: &str = lit.next().unwrap().split_once(":").unwrap().0;
        let name1_or_num = lit.next().unwrap();
        if let Some(op) = lit.next() {
            let op: Op = op.parse().unwrap();
            let name1 = name1_or_num;
            let name2 = lit.next().unwrap();
            monkeys.insert(name, Monkey::Expr(name1, op, name2));
        } else {
            let num: f64 = name1_or_num.parse().unwrap();
            monkeys.insert(name, Monkey::Num(num));
        }
    });

    println!(
        "part 1: {}",
        eval(build_monkeys("root", &monkeys, false, 0f64))
    );

    // part 2
    let (root, b) = match build_monkeys("root", &monkeys, true, 0f64) {
        Monkeys::Num(_) => todo!(),
        Monkeys::Expr(mn1, _, _) => {
            let n11 = eval(*mn1);
            match build_monkeys("root", &monkeys, true, 100000f64) {
                Monkeys::Num(_) => todo!(),
                Monkeys::Expr(mn11, _, mn22) => {
                    let n1 = eval(*mn11);
                    let n2 = eval(*mn22);
                    if n11 == n2 {
                        (n1, true)
                    } else {
                        (n2, false)
                    }
                }
            }
        }
    };

    // binary search
    let mut l: f64 = 0f64;
    let mut r: f64 = 1_000_000_000_000_000f64;
    while l <= r {
        let m = ((l + r) / 2f64).floor();
        let j = match build_monkeys("root", &monkeys, true, m) {
            Monkeys::Expr(n1, _, n2) => {
                if b {
                    eval(*n2)
                } else {
                    eval(*n1)
                }
            }
            _ => unreachable!(),
        };
        let s = root - j;
        if s < 0f64 {
            l = m + 1f64;
        } else if s > 0f64 {
            r = m - 1f64;
        } else {
            println!("part 2: {m}");
            break;
        }
    }
}
