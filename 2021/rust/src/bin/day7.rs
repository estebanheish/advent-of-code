use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/day7.txt").unwrap();

    let mut crabs: Vec<u32> = input
        .strip_suffix("\n")
        .unwrap()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();

    crabs.sort();
    let left_crab: &u32 = crabs.first().unwrap();
    let right_crab: &u32 = crabs.last().unwrap();

    let mut fuel: u32 = u32::max_value();
    for i in *left_crab..*right_crab {
        let fuel_to_here = crabs
            .iter()
            .map(|&crab| {
                let x = (crab as i32 - i as i32).abs();
                (x * (x + 1) / 2) as u32
            })
            .sum();

        if fuel_to_here < fuel {
            fuel = fuel_to_here;
        }
    }

    println!("{}", fuel);
}
