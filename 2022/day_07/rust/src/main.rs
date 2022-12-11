use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input.txt");

    let mut current_path: Vec<String> = Vec::new();
    let mut filesize: HashMap<String, usize> = HashMap::new();
    for k in input.lines() {
        if k.starts_with("$ cd") {
            let dir = k.split(" ").skip(2).next().unwrap();
            if dir == ".." {
                current_path.pop();
            } else {
                current_path.push(dir.to_owned());
            }
        } else {
            let n = k.split(" ").next().unwrap();
            if let Ok(n) = n.parse::<usize>() {
                for i in 1..current_path.len() + 1 {
                    let key: String = current_path.iter().map(|s| s.clone()).take(i).collect();
                    filesize.entry(key).and_modify(|x| *x += n).or_insert(n);
                }
            }
        }
    }

    let fs = filesize.iter().map(|(_, v)| v);
    let used = filesize.get("/").unwrap();

    println!("{}", fs.clone().filter(|&&n| n <= 100000).sum::<usize>());
    println!("{}", fs.filter(|&&v| used - v <= 40000000).min().unwrap());
}
