use std::str::FromStr;

use aoc_2022::day_02_b;

fn main() {
    let result = include_str!("../../inputs/day_02.txt")
        .lines()
        .map(|m| day_02_b::Match::from_str(m).unwrap().score())
        .sum::<i16>();

    println!("result: {:?}", result);
}

// RESULT: 13187
