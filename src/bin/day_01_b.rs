use aoc_2022::day_01;

fn main() {
    let mut result = day_01::parse_input(include_str!("../../inputs/day_01.txt"));
    result.sort_by(|a, b| b.cmp(a));

    println!("result: {:?}", result.into_iter().take(3).sum::<usize>());
}

// RESULT: 208180
