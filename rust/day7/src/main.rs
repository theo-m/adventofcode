fn main2() {
    let mut n = include_str!("./../../../inputs/day7.txt")
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect::<Vec<i32>>();
    let pos = n.len() / 2;
    let med = *n.select_nth_unstable(pos).1;
    println!("{}", n.iter().map(|x| (*x - med).abs()).sum::<i32>())
}

fn main() {
    let n = include_str!("./../../../inputs/day7.txt")
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>();
    let mean = n.iter().sum::<i32>() / n.len() as i32;
    println!("{}",
             vec![mean, mean + 1].iter()
                 .map(|x| n.iter()
                     .map(|t| (x - *t).abs())
                     .map(|d| d * (d + 1) / 2)
                     .sum::<i32>()
                 )
                 .min().unwrap()
    )
}
