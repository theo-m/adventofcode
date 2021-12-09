use std::fs;

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
    where
        T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

fn to_u32(n: Vec<u8>) -> u32 {
    let mut out: u32 = 0;
    n.iter().cloned().enumerate().for_each(|(idx, b)| {
        out += (1 << (n.len() - idx - 1)) * (b as u32);
    });
    out
}

fn main() {
    let content = fs::read_to_string("day3.txt").unwrap();
    let numbers: Vec<Vec<u8>> = content.split("\n")
        .filter(|line| !line.trim().is_empty())
        .map(|line|
            line
                .trim()
                .split("")
                .filter(|c| !c.is_empty())
                .map(|s| s.parse().unwrap())
                .collect())
        .collect();

    let bits = transpose(numbers.clone());
    let (mut gamma, mut epsilon) = (0, 0);
    bits
        .iter()
        .cloned()
        .enumerate()
        .for_each(|(idx, v)| {
            let (zeros, ones): (Vec<u8>, Vec<u8>) = v
                .iter()
                .copied()
                .partition(|b| b.clone() == 0);
            let most_common = if zeros.len() > ones.len() { 0 } else { 1 };

            gamma += (1 << (bits.len() - idx - 1)) * most_common;
            epsilon += (1 << (bits.len() - idx - 1)) * (1 - most_common);
        });

    println!("part1: {},{} => {}", gamma, epsilon, gamma * epsilon); // 1025636

    fn counter(numbers: Vec<Vec<u8>>, is_oxy: bool) -> u32 {
        let mut idx = 0;
        let mut winners: Vec<Vec<u8>> = numbers.clone();

        while winners.len() != 1 {
            let tr = transpose(winners.clone());
            let curr = tr[idx].clone();
            let (zeros, ones): (Vec<u8>, Vec<u8>) = curr
                .clone()
                .iter()
                .cloned()
                .partition(|b| b.clone() == 0);
            let mut most_common: u8 = if zeros.len() > ones.len() { 0 } else { 1 };
            if !is_oxy { most_common = 1 - most_common }

            let winners_ids: Vec<usize> = curr.clone().iter().cloned().enumerate().filter(|(_, i)| i.clone() == most_common).map(|(i, _)| i).collect();

            winners = winners.clone().iter().enumerate().filter(|(i, _)| winners_ids.contains(i)).map(|(_, v)| v.clone()).collect();
            idx += 1;
        }

        to_u32(winners.first().unwrap().clone())
    }

    let (oxy, co2) = (counter(numbers.clone(), true), counter(numbers.clone(), false));
    println!("part2: {},{} => {}", oxy, co2, oxy * co2); // 793873
}