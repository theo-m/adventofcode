fn main1() {
    let mut fishies = include_str!("day6.txt")
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

fn main() {
    let fishies = include_str!("day6.txt")
        .split_once("\n").unwrap().0
        .split(",").map(|ns| ns.parse().unwrap())
        .collect::<Vec<u32>>();

    let init_map = fishies.iter().fold(vec![0u64; 9], |mut acc, v| {
        acc[*v as usize] += 1;
        acc
    });
    println!("{:?}", init_map);
    let cnt = (0..256).fold(init_map, |mut acc, _| {
        println!("{:?}", acc);
        let day0 = acc[0];
        acc.clone().iter().enumerate().rev().for_each(|(day, num)| {
            println!("{}:{}", day, num);
            if day == 0 {
                acc[8] = day0;
                acc[6] += day0;
            } else {
                acc[day - 1] = *num;
            }
        });
        println!("{:?}", acc);
        acc
    })
        .iter().sum::<u64>();
    println!("{:?}", cnt);
}
