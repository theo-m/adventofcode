const OPENERS: &str = "[{(<";
const CLOSERS: &str = "]})>";

fn input() -> Vec<Vec<char>> {
    include_str!("../../inputs/day10.test.txt")
        .lines()
        .map(|s| s.chars().collect())
        .collect()
}

fn part1() {
    let chunks = input();
    let mut closers = vec![];

    chunks.iter().for_each(|seq| {
        let mut opened: Vec<(usize, char)> = vec![];

        seq.iter().enumerate().for_each(|(i, c)| {
            println!("curr pos {}={}, opened={:?}", i, c, opened);
            if let Some(i) = CLOSERS.find(|cc| cc == *c) {
                println!("..found a closer");
                if let Some(idxx) = opened.iter().rposition(|x| x.1 == OPENERS.chars().nth(i).unwrap()) {
                    if idxx == opened.len() - 1 {
                        opened.remove(idxx);
                        println!("..with an opener at {}", idxx);
                    } else {
                        closers.push(c);
                        println!("xx ERROR {:?}", closers);
                        return;
                    }
                } else {
                    closers.push(c);
                    println!("xx ERROR {:?}", closers);
                    return;
                }
            } else {
                opened.push((i, *c));
            }
        });
    });

    println!("p1: {:?}", closers);
}

fn part2() {}

fn main() {
    part1();
    part2();
}
