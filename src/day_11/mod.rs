use std::{collections::VecDeque, str::FromStr};

type OperationFn = fn(usize) -> usize;

fn parse_operation(operation: &str) -> OperationFn {
    unimplemented!()
}

pub struct Monkey {
    pub items: VecDeque<usize>,
    pub operation: OperationFn,
    pub test: (usize, usize, usize),
}

impl FromStr for Monkey {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items = s
            .lines()
            .nth(1)
            .expect("Could not find items")
            .split_once(":")
            .expect("Could not find items")
            .1
            .trim()
            .split(",")
            .map(|s| s.trim().parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        unimplemented!()
    }
}
