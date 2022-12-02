use aoc_2022::day_01;

fn main() {
    let result = day_01::parse_input(include_str!("../../inputs/day_01.txt"))
        .into_iter()
        .max();

    match result {
        Some(cal) => println!("result: {}", cal),
        None => println!("Error"),
    }
}

// RESULT: 69912
