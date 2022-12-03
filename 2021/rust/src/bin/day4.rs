use std::fs::read_to_string;

type Carton = Vec<Vec<Option<u32>>>;
type Cartones = Vec<Carton>;

fn main() {
    let input = read_to_string("input/day4.txt").unwrap();

    let (numeros, cartones) = input.split_once("\n\n").unwrap();

    let mut cartones: Cartones = cartones
        .split("\n\n")
        .map(|carton| {
            carton
                .split("\n")
                .map(|linea| {
                    linea
                        .split_whitespace()
                        .map(|numero| Some(numero.parse().unwrap()))
                        .collect()
                })
                .filter(|x: &Vec<_>| !x.is_empty())
                .collect()
        })
        .collect();

    let numeros: Vec<u32> = numeros.split(",").map(|n| n.parse().unwrap()).collect();

    let mut last: u32 = 0;
    let mut juan = true;
    for numero in numeros {
        mueve(&mut cartones, numero);
        let _ = win(&cartones);
        for i in win(&cartones) {
            let c: u32 = cartones[i].iter().flatten().map(|n| n.unwrap_or(0)).sum();

            last = c * numero;
            cartones.remove(i);
            if juan {
                println!("{}", last);
                juan = false;
            }
        }
    }
    println!("{}", last);
}

fn win(cartones: &Cartones) -> Vec<usize> {
    let mut winners: Vec<usize> = vec![];
    for (j, carton) in cartones.iter().enumerate() {
        for i in 0..5 {
            let line = &carton[i];
            let mut column = vec![];
            for line in carton {
                column.push(line[i]);
            }

            let l = line.iter().all(|&n| n == None);
            let c = column.iter().all(|&n| n == None);

            if l || c {
                winners.push(j);
            } else {
                continue;
            }
        }
    }
    winners.dedup();
    winners.sort_unstable_by(|x, y| y.cmp(x));
    winners
}

fn mueve(cartones: &mut Cartones, numero: u32) -> () {
    for carton in cartones {
        for linea in carton {
            for n in linea {
                if let Some(nn) = n {
                    if nn == &numero {
                        *n = None
                    }
                }
            }
        }
    }
}
