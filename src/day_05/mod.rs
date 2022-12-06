use regex::Regex;
use std::{collections::HashMap, str::FromStr};

#[derive(Clone, Debug)]
pub struct Cargo(HashMap<usize, Vec<char>>);

#[derive(Clone, Debug, PartialEq)]
pub struct Step {
    crates: usize,
    from: usize,
    to: usize,
}

impl FromStr for Step {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"move (?P<crates>\d+) from (?P<from>\d+) to (?P<to>\d+)").unwrap();
        }

        return match RE.captures(s) {
            Some(cap) => Ok(Step {
                crates: cap["crates"].parse().unwrap(),
                from: cap["from"].parse().unwrap(),
                to: cap["to"].parse().unwrap(),
            }),
            _ => Err(()),
        };
    }
}

impl FromStr for Cargo {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cargo_map: HashMap<usize, Vec<char>> = HashMap::new();

        s.lines()
            .filter(|l| l.trim().starts_with('['))
            .rev()
            .for_each(|l| {
                l.chars()
                    .skip(1)
                    .step_by(4)
                    .enumerate()
                    .filter(|(_, c)| *c != ' ')
                    .for_each(|(i, c)| {
                        cargo_map
                            .entry(i + 1)
                            .or_insert(Vec::<char>::new())
                            .push(c.to_ascii_lowercase());
                    })
            });

        return Ok(Cargo(cargo_map));
    }
}

impl Cargo {
    pub fn apply(&mut self, step: Step) {
        for _ in 0..step.crates {
            match self.0.entry(step.from).or_insert(Vec::new()).pop() {
                Some(c) => self.0.entry(step.to).or_insert(Vec::new()).push(c),
                _ => panic!("what"),
            };
        }
    }

    pub fn apply_air(&mut self, step: Step) {
        let mut crates = Vec::<char>::new();

        for _ in 0..step.crates {
            match self.0.entry(step.from).or_insert(Vec::new()).pop() {
                Some(c) => crates.push(c),
                _ => panic!("what"),
            };
        }

        crates.reverse();

        self.0
            .entry(step.to)
            .or_insert(Vec::new())
            .append(&mut crates);
    }

    pub fn heads(&self) -> Vec<char> {
        let mut heads = Vec::<char>::new();
        for i in 0..self.0.len() {
            match self.0.get(&(i + 1)) {
                Some(c) => heads.push(*c.last().unwrap()),
                None => {}
            }
        }

        return heads;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cargo_from_str() {
        let s = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3";

        let cargo = s.parse::<Cargo>().unwrap();

        assert_eq!(cargo.0.get(&1).unwrap().to_vec(), vec!['z', 'n']);
        assert_eq!(cargo.0.get(&2).unwrap().to_vec(), vec!['m', 'c', 'd']);
        assert_eq!(cargo.0.get(&3).unwrap().to_vec(), vec!['p']);
    }

    #[test]
    fn test_step_from_str() {
        let test_cases = [
            (
                "move 1 from 2 to 1",
                Step {
                    crates: 1,
                    from: 2,
                    to: 1,
                },
            ),
            (
                "move 1 from 2 to 3",
                Step {
                    crates: 1,
                    from: 2,
                    to: 3,
                },
            ),
            (
                "move 100 from 1 to 99999",
                Step {
                    crates: 100,
                    from: 1,
                    to: 99999,
                },
            ),
        ];

        for (str, step) in test_cases {
            assert_eq!(str.parse::<Step>(), Ok(step));
        }
    }

    #[test]
    fn test_cargo_apply_step() {
        let mut cargo = Cargo(
            [
                (1, ['z', 'n'].to_vec()),
                (2, ['m', 'c', 'd'].to_vec()),
                (3, ['p'].to_vec()),
            ]
            .into_iter()
            .collect(),
        );

        cargo.apply(Step {
            crates: 1,
            from: 2,
            to: 1,
        });

        assert_eq!(cargo.0.get(&1).unwrap().to_vec(), vec!['z', 'n', 'd']);
        assert_eq!(cargo.0.get(&2).unwrap().to_vec(), vec!['m', 'c']);
        assert_eq!(cargo.0.get(&3).unwrap().to_vec(), vec!['p']);

        cargo.apply(Step {
            crates: 3,
            from: 1,
            to: 3,
        });

        assert_eq!(cargo.0.get(&1).unwrap().to_vec(), Vec::<char>::new());
        assert_eq!(cargo.0.get(&2).unwrap().to_vec(), vec!['m', 'c']);
        assert_eq!(cargo.0.get(&3).unwrap().to_vec(), vec!['p', 'd', 'n', 'z']);

        cargo.apply(Step {
            crates: 2,
            from: 2,
            to: 1,
        });

        assert_eq!(cargo.0.get(&1).unwrap().to_vec(), vec!['c', 'm']);
        assert_eq!(cargo.0.get(&2).unwrap().to_vec(), Vec::<char>::new());
        assert_eq!(cargo.0.get(&3).unwrap().to_vec(), vec!['p', 'd', 'n', 'z']);

        cargo.apply(Step {
            crates: 1,
            from: 1,
            to: 2,
        });

        assert_eq!(cargo.0.get(&1).unwrap().to_vec(), vec!['c']);
        assert_eq!(cargo.0.get(&2).unwrap().to_vec(), vec!['m']);
        assert_eq!(cargo.0.get(&3).unwrap().to_vec(), vec!['p', 'd', 'n', 'z']);
    }

    #[test]
    fn test_cargo_heads() {
        let cargo = Cargo(
            [
                (1, ['z', 'n'].to_vec()),
                (2, ['m', 'c', 'd'].to_vec()),
                (3, ['p'].to_vec()),
            ]
            .into_iter()
            .collect(),
        );

        assert_eq!(cargo.heads(), vec!['n', 'd', 'p']);
    }
}
