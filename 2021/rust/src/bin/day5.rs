#![feature(slice_group_by)]
use std::fs::read_to_string;

type Punto = (u32, u32);
type Trazo = (Punto, Punto);

fn main() {
    let input = read_to_string("input/day5.txt").unwrap();

    let trazos: Vec<Trazo> = input
        .lines()
        .map(|p| {
            p.split(" -> ")
                .map(|e| {
                    e.split(",")
                        .map(|n| n.parse().unwrap())
                        .collect::<Vec<u32>>()
                })
                .collect::<Vec<Vec<u32>>>()
        })
        .map(|p| ((p[0][0], p[0][1]), (p[1][0], p[1][1])))
        .collect();

    let mut puntos: Vec<Punto> = Vec::new();
    for ((x, y), (z, w)) in trazos {
        puntos.push((x, y));
        let mut xm = x.clone();
        let mut ym = y.clone();
        while xm != z || ym != w {
            match xm.cmp(&z) {
                std::cmp::Ordering::Less => xm += 1,
                std::cmp::Ordering::Greater => xm -= 1,
                std::cmp::Ordering::Equal => (),
            }
            match ym.cmp(&w) {
                std::cmp::Ordering::Less => ym += 1,
                std::cmp::Ordering::Greater => ym -= 1,
                std::cmp::Ordering::Equal => (),
            }
            puntos.push((xm, ym));
        }
    }

    puntos.sort();
    let out = puntos
        .group_by(|a, b| a == b)
        .filter(|x| x.len() > 1)
        .count();

    println!("{}", out);
}
