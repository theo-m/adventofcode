fn main() {
    let str = include_str!("day5.txt");
    let mut overlaps = 0;
    let count = str.lines()
        .map(|l| {
            let (left, right) = l.split_once(" -> ").unwrap();
            let (xas, yas) = left.split_once(",").unwrap();
            let (xbs, ybs) = right.split_once(",").unwrap();
            let nums: Vec<isize> = vec![xas, yas, xbs, ybs].iter().map(|s| s.parse::<isize>().unwrap()).collect();
            (nums[0], nums[1], nums[2], nums[3])
        })
        .fold(vec![0; 1000 * 1000], |mut acc, (xa, ya, xb, yb)| {
            println!("{},{}-{},{}", xa, ya, xb, yb);
            if xa == xb {
                (ya.min(yb)..=yb.max(ya)).for_each(|y| {
                    if acc[(xa + 1000 * ya) as usize] == 1 {
                        overlaps += 1;
                    }
                    acc[(xa + 1000 * y) as usize] += 1
                });
            } else if ya == yb {
                (xa.min(xb)..=xb.max(xa)).for_each(|x| {
                    if acc[(xa + 1000 * ya) as usize] == 1 {
                        overlaps += 1;
                    }
                    acc[(x + 1000 * ya) as usize] += 1
                });
            } else {
                assert_eq!((yb - ya).abs(), (xb - xa).abs());
                (0..=(yb - ya).abs()).for_each(|inc| {
                    let (xinc, yinc) = (inc * (xb - xa).signum(), inc * (yb - ya).signum());
                    let idx = (xa + xinc + 1000 * (ya + yinc)) as usize;
                    println!("{}, ({},{}) vis:({})", inc, xa + xinc, ya + yinc, acc[idx]);
                    if acc[idx] == 1 {
                        overlaps += 1;
                    }
                    acc[idx] += 1;
                });
            }
            acc
        })
        .iter().cloned()
        .filter(|c| *c >= 2).count();

    println!("{:?}, {}", count, overlaps); // 21942 too high, 11792 too low, 11811 too low
}
