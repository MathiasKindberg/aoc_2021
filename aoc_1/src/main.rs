use std::io::prelude::*;

fn input() -> Vec<usize> {
    let stdin = std::io::stdin();
    stdin
        .lock()
        .lines()
        .into_iter()
        .filter_map(|line| line.ok().and_then(|item| item.parse::<usize>().ok()))
        .collect()
}

fn one(input: &Vec<usize>) -> usize {
    input.windows(2).filter(|items| items[0] < items[1]).count()
}

fn two(input: &Vec<usize>) -> usize {
    input
        .windows(4)
        .filter(|items| items[0..3].iter().sum::<usize>() < items[1..4].iter().sum())
        .count()
}

fn main() {
    let input = input();
    println!("One: {}", one(&input));
    println!("Two: {}", two(&input));
}
