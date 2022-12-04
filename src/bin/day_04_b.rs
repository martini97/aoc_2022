use aoc_2022::{day_04::Assignment, utils::read_input};

fn solve(input: String) -> usize {
    input
        .lines()
        .filter_map(|s| match s.parse::<Assignment>().unwrap().has_overlap() {
            true => Some(()),
            _ => None,
        })
        .count()
}

fn main() {
    println!("result: {:?}", solve(read_input(4, false)));
}

// RESULT: 815

#[cfg(test)]
mod tests {
    use aoc_2022::utils::read_input;

    use super::*;

    #[test]
    fn solve_test_input() {
        assert_eq!(solve(read_input(4, true)), 4);
    }
}
