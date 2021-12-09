use std::collections::HashMap;

fn neighbors(w: usize, h: usize, i: usize, j: usize) -> Vec<(usize, usize)> {
    [
        Some((i, j)),
        if i == 0 { None } else { Some((i - 1, j)) },
        if i == w - 1 { None } else { Some((i + 1, j)) },
        if j == 0 { None } else { Some((i, j - 1)) },
        if j == h - 1 { None } else { Some((i, j + 1)) },
    ].iter().filter_map(|x| *x).collect()
}

fn part1() {
    let m: Vec<Vec<u8>> = include_str!("../../../inputs/day9.2.txt")
        .lines()
        .map(|line|
            line.trim().split("")
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<u8>().unwrap())
                .collect())
        .collect();
    let mut lows = vec![];
    let (w, h) = (m.len(), m[0].len());
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
    println!("{:?}", lows.iter().map(|(_, pos)| pos).collect::<Vec<_>>());
    println!("{:?}", lows.iter().map(|(x, _)| 1 + *x as u32).sum::<u32>());
}

fn main() {
    let m: Vec<Vec<u8>> = include_str!("../../../inputs/day9.txt")
        .lines()
        .map(|line|
            line.trim().split("")
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<u8>().unwrap())
                .collect())
        .collect();
    let (w, h) = (m.len(), m[0].len());

    let mut basins = HashMap::new();
    m.iter()
        .flatten()
        .enumerate()
        .filter_map(|(i, d)| if *d == 9 { None } else { Some((i / h, i % h)) })
        .for_each(|(x, y)| {
            let ns = neighbors(w, h, x, y);
            let reprs = ns.iter()
                .filter_map(|pos| {
                    let b = basins.clone();
                    b.get(pos).and_then(|n: &usize| Some((*pos, n.clone())))
                })
                .collect::<Vec<((usize, usize), usize)>>();

            if reprs.is_empty() {
                basins.insert((x, y), (x + h * y));
            } else {
                let repr = reprs[0].1.clone();
                reprs.clone().iter().for_each(|(key, _)| { *basins.get_mut(key).unwrap() = repr; });
                basins.insert((x, y), repr);
            }
        });
    let mut basin_size = basins.iter().fold(HashMap::<usize, Vec<(usize, usize)>>::new(), |mut acc, (pos, repr)| {
        if let Some(v) = acc.get_mut(repr) {
            v.push(*pos)
        } else {
            acc.insert(*repr, vec![*pos]);
        }
        acc
    }).iter().map(|b| b.1.len()).collect::<Vec<_>>();
    basin_size.sort();
    basin_size.reverse();
    println!("{:?} ",basin_size.len());
    println!("{:?} -> {}",basin_size.iter().take(3).collect::<Vec<_>>(), basin_size.iter().take(3).product::<usize>());
}