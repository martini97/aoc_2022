use aoc_2022::{day_06::get_first_marker_at, utils::read_input};

fn solve(input: String) -> usize {
    return get_first_marker_at(&input, 14);
}

fn main() {
    println!("result: {:?}", solve(read_input(6, false)))
}

// RESULT: 2202
