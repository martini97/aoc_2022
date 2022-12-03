use std::collections::HashSet;
use std::iter::FromIterator;
use std::str::FromStr;

pub struct Rucksack(HashSet<u8>, HashSet<u8>);

impl FromStr for Rucksack {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_at(s.len() / 2);
        let a: HashSet<u8> = HashSet::from_iter(a.as_bytes().to_vec());
        let b: HashSet<u8> = HashSet::from_iter(b.as_bytes().to_vec());
        return Ok(Rucksack(a, b));
    }
}

impl Rucksack {
    pub fn get_item_type(&self) -> char {
        let intersection = self.0.intersection(&self.1).collect::<Vec<&u8>>();
        return *intersection[0] as char;
    }

    pub fn joined(&self) -> HashSet<u8> {
        return self.0.union(&self.1).copied().collect();
    }
}

pub fn get_item_type_priority(item: char) -> u8 {
    match item {
        'a'..='z' => item as u8 - 96,
        'A'..='Z' => item as u8 - 38,
        _ => panic!("{} is not a valid item", item),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rucksack_parse_from_str() {
        let test_cases = [
            ["vJrwpWtwJgWrhcsFMMfFFhFp", "vJrwpWtwJgWr", "hcsFMMfFFhFp"],
            [
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "jqHRNqRjqzjGDLGL",
                "rsFMfFZSrLrFZsSL",
            ],
            ["PmmdzqPrVvPwwTWBwg", "PmmdzqPrV", "vPwwTWBwg"],
        ];

        for case in test_cases {
            let [rucksack_str, first, second] = case;
            let rucksack = Rucksack::from_str(rucksack_str).unwrap();
            assert_eq!(rucksack.0, HashSet::from_iter(first.as_bytes().to_vec()));
            assert_eq!(rucksack.1, HashSet::from_iter(second.as_bytes().to_vec()));
        }
    }

    #[test]
    fn rucksack_get_item_type() {
        let test_cases = [
            ("vJrwpWtwJgWrhcsFMMfFFhFp", 'p'),
            ("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", 'L'),
            ("PmmdzqPrVvPwwTWBwg", 'P'),
            ("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn", 'v'),
            ("ttgJtRGJQctTZtZT", 't'),
            ("CrZsJsPPZsGzwwsLwLmpwMDw", 's'),
        ];

        for case in test_cases {
            let (rucksack_str, expected_item_type) = case;
            let rucksack = Rucksack::from_str(rucksack_str).unwrap();
            assert_eq!(rucksack.get_item_type(), expected_item_type);
        }
    }

    #[test]
    fn test_get_item_type_priority() {
        let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
        for (i, c) in chars.chars().enumerate() {
            assert_eq!(get_item_type_priority(c), i as u8 + 1);
        }
    }
}
