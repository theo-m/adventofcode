use std::collections::HashMap;

fn main() {
    let content = include_str!("../../../inputs/day4.txt");
    let (roll_str, boards_str) = content.split_once("\n\n").unwrap();
    let roll: Vec<u32> = roll_str.split(",").map(|n| n.parse().unwrap()).collect();
    let boards: Vec<HashMap<(usize, usize), u32>> = boards_str
        .split("\n\n")
        .map(|bs| bs
            .lines()
            .enumerate()
            .map(|(i, row)| row
                .split_whitespace()
                .enumerate()
                .map(|(j, n)| ((i, j), n.parse::<u32>().unwrap()))
                .collect::<Vec<((usize, usize), u32)>>()
            )
            .flatten()
            .collect()
        )
        .collect();
    let mut marks: Vec<HashMap<(usize, usize), u8>> = boards.clone().iter()
        .map(|e| e.iter().map(|(key, _)| (key.clone(), 0)).collect()).collect();

    let mut out = vec![];
    for n in roll {
        boards.iter().enumerate()
            .for_each(|(i, b)|
                if !out.contains(&i) {
                    b.iter()
                        .for_each(|(k, v)| if *v == n {
                            *marks[i].get_mut(k).unwrap() = 1;
                        })
                }
            );
        for (i, m) in marks.iter().enumerate() {
            if out.contains(&i) {continue;}
            let keys: Vec<(usize, usize)> = m.iter().filter(|(k, v)| **v == 1).map(|(k, _)| *k).collect();
            let rows = keys.clone().iter().fold(vec![0; 5], |mut acc, v| {
                acc[(*v).0] += 1;
                acc
            });
            let cols = keys.clone().iter().fold(vec![0; 5], |mut acc, v| {
                acc[(*v).1] += 1;
                acc
            });
            if rows.iter().any(|s| (*s) == 5) || cols.iter().any(|s| (*s) == 5) {
                println!(
                    "{} board has finished (n={}): {} ({:?})",
                    i,
                    n,
                    (n as u32) * boards[i].iter().map(|(k, v)|
                        if *m.get(k).unwrap() == 0 { *v as u32 } else { 0 }
                    ).sum::<u32>(),
                    boards[i].iter().map(|(k, v)|
                        if *m.get(k).unwrap() == 0 { *v as u32 } else { 0 }
                    ).sum::<u32>()
                );
                out.push(i);
            }
        }
    }
}