use std::collections::{HashMap, HashSet};
use std::str::Chars;

fn main() {
    let numbers: HashMap<u8, Vec<char>> = HashMap::from_iter(vec![
        (0, "abcefg".chars().collect::<Vec<char>>()), (1, "cf".chars().collect::<Vec<char>>()),
        (2, "acdeg".chars().collect::<Vec<char>>()), (3, "acdfg".chars().collect::<Vec<char>>()),
        (4, "bcdf".chars().collect::<Vec<char>>()), (5, "abdfg".chars().collect::<Vec<char>>()),
        (6, "abdefg".chars().collect::<Vec<char>>()), (7, "acf".chars().collect::<Vec<char>>()),
        (8, "abcdefg".chars().collect::<Vec<char>>()), (9, "abcdfg".chars().collect::<Vec<char>>()),
    ]);
    fn inv(v: &Vec<char>) -> Vec<char> {
        "abcdefg".chars().filter(|c| !v.contains(c)).collect()
    }

    let displays = include_str!("day8.test.txt")
        .lines()
        .map(|lin| {
            let (left, right) = lin.split_once(" | ").unwrap();
            (left.split_whitespace().collect::<Vec<&str>>(), right.split_whitespace().collect::<Vec<&str>>())
        })
        .collect::<Vec<_>>();
    println!("{:?}", displays);
    displays.iter().for_each(|(digits, _secrets)| {
        let mut segments = "abcdefg".chars().map(|c| (c, HashSet::new())).collect::<HashMap<char, HashSet<char>>>();
        let by_length = digits.iter().fold(HashMap::<usize, Vec<&str>>::new(), |mut acc, v| {
            acc.entry(v.len()).or_insert(vec![]).push(v);
            acc
        });

        let one = by_length.get(&2).unwrap()[0];
        one.chars().for_each(|c| {
            let one_repr = numbers.get(&1).unwrap();
            one_repr.iter().for_each(|r| { segments.get_mut(&r).unwrap().insert(c); });
            inv(one_repr).iter().for_each(|r| { segments.get_mut(&r).unwrap().remove(&c); });
        });

        let four = by_length.get(&4).unwrap()[0];
        four.chars().for_each(|c| {
            numbers.get(&4).unwrap().iter().for_each(|r| { segments.get_mut(&r).unwrap().insert(c); })
        });

        let seven = by_length.get(&3).unwrap()[0];
        seven.chars().for_each(|c| {
            numbers.get(&7).unwrap().iter().for_each(|r| { segments.get_mut(&r).unwrap().insert(c); })
        });

        println!("{:?}", by_length.get(&2).unwrap().first().unwrap());
        println!("{:?}", segments);
    })
}
