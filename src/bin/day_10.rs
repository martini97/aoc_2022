use std::str::FromStr;

use aoc_2022::{
    day_10::{Instruction, CPU},
    utils::read_input,
};

fn main() {
    let instructions = read_input(10, false)
        .trim()
        .lines()
        .map(|s| Instruction::from_str(s).unwrap())
        .collect::<Vec<Instruction>>();

    let is_interesting = |i: usize| (i as isize - 20) % 40 == 0;

    let mut cpu = CPU::new(instructions);
    let mut signal_strength = 0;

    for i in 1..240 {
        if is_interesting(i) {
            signal_strength += cpu.get_signal_strength();
        }
        cpu.tick();
    }

    println!("{:?}", signal_strength);
    cpu.draw();
}
