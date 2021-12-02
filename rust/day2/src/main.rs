use std::fs;
use std::str::FromStr;

#[derive(Debug)]
enum Direction {
    Forward,
    Up,
    Down,
}

#[derive(Debug)]
struct Move {
    dir: Direction,
    step: u32,
}

#[derive(Debug)]
enum MoveErr {
    WrongFormat,
    WrongDirection,
}

impl FromStr for Move {
    type Err = MoveErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items: Vec<&str> = s.split(" ").collect();
        if items.len() != 2 {
            return Err(MoveErr::WrongFormat);
        }

        let dir: Direction;
        match items.first().unwrap().trim() {
            "forward" => dir = Direction::Forward,
            "up" => dir = Direction::Up,
            "down" => dir = Direction::Down,
            _ => return Err(MoveErr::WrongDirection)
        }

        let step: u32 = items[1].parse().unwrap();

        let mov = Move { dir, step };
        Ok(mov)
    }
}

fn main() {
    let content = fs::read_to_string("../../inputs/day2.txt").unwrap();
    let moves: Vec<Move> = content.split("\n")
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.trim().parse::<Move>().unwrap())
        .collect();

    let (mut x, mut y) = (0, 0);
    moves.iter().for_each(|m|
        match m.dir {
            Direction::Forward => { x += m.step }
            Direction::Up => { y -= m.step }
            Direction::Down => { y += m.step }
        }
    );
    println!("part1: {},{} => mult = {}", x, y, x * y);

    let (mut x, mut y, mut aim) = (0, 0, 0);
    moves.iter().for_each(|m|
        match m.dir {
            Direction::Forward => {
                x += m.step;
                y += m.step * aim
            }
            Direction::Up => { aim -= m.step }
            Direction::Down => { aim += m.step }
        }
    );
    println!("part2: {},{}(aim={}) => mult = {}", x, y, aim, x * y);
}
