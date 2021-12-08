fn main() {
    let mut fishies = include_str!("../../../inputs/day6.txt")
        .split_once("\n").unwrap().0
        .split(",").map(|ns| ns.parse().unwrap())
        .collect::<Vec<u32>>();
    (0..80).for_each(|_d| {
        fishies = fishies.iter().enumerate().fold(fishies.clone(), |mut curr, (i, x)| {
            if *x == 0 {
                curr[i] = 6;
                curr.push(8);
            } else {
                curr[i] -= 1
            }
            curr
        });
        println!("day {}", _d);
    });
    println!("{:?}", fishies.len())
}
