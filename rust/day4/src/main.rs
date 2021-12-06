use std::borrow::BorrowMut;
use std::fs;
use std::ops::Deref;

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
    where
        T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}


fn main() {
    let content = fs::read_to_string("../../inputs/day4.test.txt").unwrap();
    let lines: Vec<&str> = content.split("\n").collect();
    let mut numbers: Vec<u32> = lines
        .first().unwrap()
        .split(",")
        .filter(|s| !s.is_empty())
        .map(|n| n.parse::<u32>().unwrap())
        .collect();
    let mut boards: Vec<Vec<Vec<u32>>> = (0..(lines.len() - 1) / 6)
        .map(|i| {
            let mut rows: Vec<Vec<u32>> = lines[i * 6 + 2..i * 6 + 7].iter()
                .map(|l|
                    l
                        .split_whitespace()
                        .map(|s| s.parse().unwrap())
                        .collect::<Vec<u32>>())
                .collect();
            let mut cols = transpose(rows.clone());
            rows.append(&mut cols);
            rows
        })
        .collect();

    let mut flag = false;
    numbers.reverse();
    while let Some(n) = numbers.pop() {
        println!("popped {:?}", n);

        boards.iter_mut().for_each(|mut board|
            board = &mut board.iter_mut()
                .map(|mut list| {
                    list = &mut list.iter().filter(|x| **x == n).map(|x| *x).collect::<Vec<u32>>();
                    if list.is_empty() { flag = true }
                    list
                })
                .collect()
        );
        if flag { break; }
    }
}