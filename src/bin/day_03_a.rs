use std::str::FromStr;

use aoc_2022::day_03::{get_item_type_priority, Rucksack};

fn main() {
    let result: u16 = include_str!("../../inputs/day_03.txt")
        .lines()
        .map(|s| {
            let rucksack = Rucksack::from_str(s).unwrap();
            return get_item_type_priority(rucksack.get_item_type()) as u16;
        })
        .sum();
    println!("result: {:?}", result);
    return;
}

// RESULT: 7850
