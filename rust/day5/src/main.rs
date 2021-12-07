
fn main() {
    let str = include_str!("../../../inputs/day5.txt");
    let count = str.lines()
        .map(|l| {
            let (left, right) = l.split_once(" -> ").unwrap();
            let (xas, yas) = left.split_once(",").unwrap();
            let (xbs, ybs) = right.split_once(",").unwrap();
            let nums: Vec<usize> = vec![xas, yas, xbs, ybs].iter().map(|s| s.parse::<usize>().unwrap()).collect();
            (nums[0].min(nums[2]), nums[1].min(nums[3]), nums[0].max(nums[2]), nums[1].max(nums[3]))
        })
        .filter(|(xmin, ymin, xmax, ymax)| xmin == xmax || ymin == ymax)
        .fold(vec![0; 1000 * 1000], |mut acc, (xmin, ymin, xmax, ymax)| {
            if xmin == xmax {
                (ymin..=ymax).for_each(|y| acc[xmin + 1000 * y] += 1);
            } else {
                (xmin..=xmax).for_each(|x| acc[x + 1000 * ymin] += 1);
            }
            acc
        })
        .iter()
        .cloned()
        .filter(|c| *c >= 2).count();
    println!("{:?}", count);
}
