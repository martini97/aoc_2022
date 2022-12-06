use aoc_2022::{
    day_05::{Cargo, Step},
    utils::read_input,
};

fn solve(input: String) -> Vec<char> {
    let (cargo, steps) = input.split_once("\n\n").unwrap();

    let mut cargo = cargo.parse::<Cargo>().unwrap();
    let steps = steps
        .lines()
        .map(|l| l.parse::<Step>().unwrap())
        .collect::<Vec<Step>>();

    for step in steps {
        cargo.apply(step);
    }

    return cargo.heads();
}

fn main() {
    println!(
        "result: {:?}",
        solve(read_input(5, false))
            .iter()
            .collect::<String>()
            .to_uppercase()
    );
}

// RESULT: FWSHSPJWM

#[cfg(test)]
mod tests {
    use aoc_2022::utils::read_input;

    use super::*;

    #[test]
    fn solve_test_input() {
        assert_eq!(solve(read_input(5, true)), vec!['c', 'm', 'z']);
    }
}
