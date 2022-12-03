use std::{collections::HashSet, str::FromStr};

use aoc_2022::day_03::{get_item_type_priority, Rucksack};

fn main() {
    let mut lines = include_str!("../../inputs/day_03.txt").lines();
    let mut result: u16 = 0;
    while let (Some(line1), Some(line2), Some(line3)) = (lines.next(), lines.next(), lines.next()) {
        let rucksack1 = Rucksack::from_str(line1).unwrap().joined();
        let rucksack2 = Rucksack::from_str(line2).unwrap().joined();
        let rucksack3 = Rucksack::from_str(line3).unwrap().joined();

        let group: HashSet<u8> = rucksack2.intersection(&rucksack3).copied().collect();
        let group: Vec<&u8> = rucksack1.intersection(&group).collect();

        result += get_item_type_priority(*group[0] as char) as u16;
    }

    println!("result: {:?}", result);

    return;
}

// RESULT: 2581
