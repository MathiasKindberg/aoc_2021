use std::io::prelude::*;

#[derive(Debug)]
enum Direction {
    Forward,
    Down,
    Up,
}

impl From<&String> for Direction {
    fn from(item: &String) -> Self {
        match item.as_str() {
            "forward" => Self::Forward,
            "down" => Self::Down,
            "up" => Self::Up,
            _ => panic!("Unknown direction"),
        }
    }
}

fn input() -> Vec<(Direction, usize)> {
    let stdin = std::io::stdin();
    stdin
        .lock()
        .lines()
        .into_iter()
        .filter_map(|line| line.ok())
        .map(|line| {
            line.split_whitespace()
                .map(|item| item.to_string())
                .collect::<Vec<String>>()
        })
        .filter_map(|items| Some(((&items[0]).into(), items[1].parse::<usize>().ok()?)))
        .collect()
}

fn one(input: &Vec<(Direction, usize)>) -> usize {
    let (horizontal, depth) = input
        .iter()
        .fold((0, 0), |(mut horizontal, mut depth), input| {
            match input.0 {
                Direction::Forward => horizontal += input.1,
                Direction::Up => depth -= input.1,
                Direction::Down => depth += input.1,
            }
            (horizontal, depth)
        });
    horizontal * depth
}

fn two(input: &Vec<(Direction, usize)>) -> usize {
    let (horizontal, depth, _) =
        input
            .iter()
            .fold((0, 0, 0), |(mut horizontal, mut depth, mut aim), input| {
                match input.0 {
                    Direction::Forward => {
                        horizontal += input.1;
                        depth += input.1 * aim
                    }
                    Direction::Up => aim -= input.1,
                    Direction::Down => aim += input.1,
                }
                (horizontal, depth, aim)
            });
    horizontal * depth
}

fn main() {
    let input = input();
    println!("One {}", one(&input));
    println!("Two {}", two(&input));
}
