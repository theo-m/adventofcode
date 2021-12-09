use std::collections::{HashMap, HashSet};

const CHARS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890πåß∂ƒ∆Ω≈ç√∫";

fn neighbors(w: usize, h: usize, i: usize, j: usize) -> Vec<(usize, usize)> {
    [
        Some((i, j)),
        if i == 0 { None } else { Some((i - 1, j)) },
        if i == w - 1 { None } else { Some((i + 1, j)) },
        if j == 0 { None } else { Some((i, j - 1)) },
        if j == h - 1 { None } else { Some((i, j + 1)) },
    ].iter().filter_map(|x| *x).collect()
}

fn input() -> Vec<Vec<u8>> {
    include_str!("../../inputs/day9.test2.txt")
        .lines()
        .map(|line|
            line.trim().split("")
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<u8>().unwrap())
                .collect())
        .collect()
}

fn part1() {
    let m = input();
    let (w, h) = (m.len(), m[0].len());

    let mut lows = vec![];
    (0..w).for_each(|i| (0..h).for_each(|j| {
        let mut neighbors = vec![
            &m[i][j],
            if j == 0 { &10 } else { m[i].get(j - 1).or(Some(&9)).unwrap() },
            if i == 0 { &10 } else { m[i - 1].get(j).or(Some(&9)).unwrap() },
            m[i].get(j + 1).or(Some(&9)).unwrap(),
            m.get(i + 1).and_then(|v| v.get(j)).or(Some(&9)).unwrap(),
        ];
        neighbors.sort();
        if **neighbors.iter().min().unwrap() == m[i][j] && *neighbors[1] != m[i][j] {
            lows.push((m[i][j], (i, j)));
        }
    }));
    println!("part1: {:?}", lows.iter().map(|(x, _)| 1 + *x as u32).sum::<u32>());
}

fn part2() {
    let m = input();
    let (w, h) = (m.len(), m[0].len());

    let mut ufind = HashMap::new();
    m.iter()
        .flatten()
        .enumerate()
        .filter_map(|(i, d)| if *d == 9 { None } else { Some((i / h, i % h)) })
        .for_each(|(x, y)| {
            let reprs = neighbors(w, h, x, y).iter()
                .filter_map(|pos| ufind.clone().get(pos).and_then(|n: &usize| Some((*pos, n.clone()))))
                .collect::<Vec<((usize, usize), usize)>>();

            if reprs.is_empty() {
                ufind.insert((x, y), x * h + y);
            } else {
                let (_, repr) = reprs[0];
                reprs.iter().for_each(|(key, _)| { *ufind.get_mut(key).unwrap() = repr; });
                ufind.insert((x, y), repr);
            }
        });

    let basins_grouped = ufind.iter()
        .fold(HashMap::<usize, Vec<(usize, usize)>>::new(), |mut acc, (pos, repr)| {
            acc.entry(*repr).or_insert(vec![]).push(*pos);
            acc
        });
    let mut basin_size = basins_grouped.iter()
        .map(|(_repr, group)| group.len())
        .collect::<Vec<_>>();
    basin_size.sort();
    basin_size.reverse();

    println!("part2: num basins = {:?} ", basin_size.len());
    println!("part2: res = {:?} -> {}", basin_size.iter().take(3).collect::<Vec<_>>(), basin_size.iter().take(3).product::<usize>());
    let cc = CHARS.chars().collect::<Vec<char>>();
    println!("{}",
             m.iter().enumerate()
                 .map(|(i, seq)|
                     seq.iter().enumerate()
                         .map(|(j, h)|
                             if *h == 9 { "■".to_string() } else { cc[ufind[&(i, j)] % cc.len()].to_string() })
                         .fold(String::new(), |acc, v| acc + &v)
                 ).fold(String::new(), |acc, v| acc + &v + "\n"));
}

fn main() {
    part1();
    part2();
}