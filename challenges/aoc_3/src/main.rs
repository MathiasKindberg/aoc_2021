use std::io::prelude::*;

fn input() -> Vec<Vec<usize>> {
    let stdin = std::io::stdin();
    stdin
        .lock()
        .lines()
        .into_iter()
        .filter_map(|line| line.ok())
        .map(|line| {
            line.chars()
                .filter_map(|char| char.to_string().parse().ok())
                .collect()
        })
        .collect()
}

fn rotate(input: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    (0..input[0].len())
        .map(|idx| input.iter().map(|c| c[idx]).collect())
        .collect()
}

fn one(input: &Vec<Vec<usize>>) -> usize {
    input.iter().map(|column| ());
    
        0
}

fn main() {
    let input = input();
    let rotated = rotate(input.clone());
    println!("Input {:#?}", input);
    println!("Rotated: {:#?}", rotated);
    println!("One {}", one(&input));
}
