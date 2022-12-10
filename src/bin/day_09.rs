use std::{collections::HashSet, str::FromStr};

use aoc_2022::{
    day_09::{Point, Rope},
    utils::read_input,
};

fn solve<const N: usize>(input: &str, mut rope: Rope<N>) -> usize {
    let mut seen: HashSet<Point> = HashSet::new();
    seen.insert(rope.tail());

    input
        .lines()
        .flat_map(|s| {
            let (dir, count) = s.split_once(" ").unwrap();
            let count = count.parse::<usize>().unwrap();
            return vec![Point::from_str(dir).unwrap(); count];
        })
        .for_each(|step| {
            rope.drag(step);
            seen.insert(rope.tail());
        });

    return seen.len();
}

fn main() {
    let input = read_input(9, false);
    let input = input.trim();

    let solution_1 = solve::<2>(input, Rope::<2>::new());
    let solution_2 = solve::<10>(input, Rope::<10>::new());

    println!("solution_1: {solution_1}");
    println!("solution_2: {solution_2}");
}
