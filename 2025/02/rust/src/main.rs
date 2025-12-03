use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut p1 = 0;
    let mut p2 = 0;

    for range in input.split(",") {
        let range = range.trim();
        let (a, b) = range.split_once('-').unwrap();
        let a: u64 = a.parse().unwrap();
        let b: u64 = b.parse().unwrap();

        'id: for id in a..=b {
            let str_id = id.to_string();
            let l = str_id.len();

            if str_id[..l / 2] == str_id[l / 2..] && l % 2 == 0 {
                p1 += id;
            }

            'p_l: for p_l in 1..=str_id.len() / 2 {
                if l % p_l != 0 {
                    continue;
                }
                for j in 0..(l / p_l) - 1 {
                    if str_id[j * p_l..j * p_l + p_l] != str_id[(j + 1) * p_l..(j + 1) * p_l + p_l]
                    {
                        continue 'p_l;
                    }
                }
                p2 += id;
                continue 'id;
            }
        }
    }

    println!("{p1}\n{p2}");
}
