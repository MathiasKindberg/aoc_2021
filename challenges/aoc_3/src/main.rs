use std::io::prelude::*;

fn input() -> Vec<Vec<u8>> {
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

fn transpose(input: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    (0..input[0].len())
        .map(|idx| input.iter().map(|c| c[idx]).collect())
        .collect()
}

fn one(input: &Vec<Vec<u8>>) -> usize {
    let gamma: Vec<u8> = transpose(input.clone())
        .iter()
        .map(|column| {
            column.iter().fold(
                std::collections::HashMap::new(),
                |mut map: std::collections::HashMap<u8, i32>, val| {
                    *map.entry(*val).or_default() += 1;
                    map
                },
            )
        })
        .filter_map(|frequencies| {
            Some(
                frequencies
                    .into_iter()
                    .max_by_key(|(_, val)| val.clone())?
                    .0,
            )
        })
        .collect();
        
    let epsilon: Vec<u8> = gamma
        .iter()
        .map(|bit| match bit {
            0 => 1,
            1 => 0,
            _ => panic!("Not bit"),
        })
        .collect();

    let gamma = usize::from_str_radix(
        &gamma
            .iter()
            .map(|item| item.to_string())
            .collect::<String>(),
        2,
    )
    .expect("Should always be valid bit string");

    let epsilon = usize::from_str_radix(
        &epsilon
            .iter()
            .map(|item| item.to_string())
            .collect::<String>(),
        2,
    )
    .expect("Should always be valid bit string");

    gamma * epsilon
}

fn main() {
    let input = input();
    println!("One {}", one(&input));
}
