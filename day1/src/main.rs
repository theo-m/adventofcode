use std::fs;
use std::path::Iter;

fn main() {
    let content = fs::read_to_string("day1.txt").unwrap();
    let sonar_readings: Vec<u32> = content.split("\n")
        .map(|it| if !it.trim().is_empty() { Some(it.trim().parse::<u32>().unwrap()) } else { None })
        .filter(|it| !it.is_none())
        .map(|it| it.unwrap())
        .collect();

    fn incr(a: &Vec<u32>) -> u32 {
        a[0..a.len() - 1].iter().zip(a[1..a.len()].iter())
            .map(|(a, b)| if b > a { 1 } else { 0 })
            .sum()
    }

    let res: u32 = incr(&sonar_readings);
    println!("part1 result: {}", res); // 1548

    let w3: Vec<u32> = sonar_readings
        .clone()
        .into_iter()
        .enumerate()
        .map(|(idx, _)|
            sonar_readings[idx..vec![idx + 3, sonar_readings.len()].iter().min().unwrap().clone()].iter().sum::<u32>()
        )
        .collect();
    println!("part2 result: {}", incr(&w3)); // 1548
}
