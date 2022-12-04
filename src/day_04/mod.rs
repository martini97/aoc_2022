use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Section(u16, u16);

#[derive(Debug, PartialEq)]
pub struct Assignment(Section, Section);

impl FromStr for Section {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once("-").unwrap();
        return Ok(Self(a.parse().unwrap(), b.parse().unwrap()));
    }
}

impl FromStr for Assignment {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once(",").unwrap();
        return Ok(Self(a.parse().unwrap(), b.parse().unwrap()));
    }
}

impl Section {
    pub fn contains(&self, other: &Self) -> bool {
        return self.0 <= other.0 && self.1 >= other.1;
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        return (self.0 <= other.0 && other.0 <= self.1)
            || (self.0 <= other.1 && other.1 <= self.1);
    }
}

impl Assignment {
    pub fn has_full_overlap(&self) -> bool {
        let Assignment(a, b) = self;

        return a.contains(b) || b.contains(a);
    }

    pub fn has_overlap(&self) -> bool {
        let Assignment(a, b) = self;

        return a.overlaps(b) || b.overlaps(a);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_section_from_str() {
        let cases = [
            ("2-4", Section(2, 4)),
            ("6-8", Section(6, 8)),
            ("2-3", Section(2, 3)),
            ("10-999", Section(10, 999)),
        ];

        for case in cases {
            let (str, expected) = case;
            assert_eq!(Section::from_str(str).unwrap(), expected);
        }
    }

    #[test]
    fn test_assignment_from_str() {
        let cases = [
            ("2-4,6-8", Assignment(Section(2, 4), Section(6, 8))),
            ("2-3,4-5", Assignment(Section(2, 3), Section(4, 5))),
            ("5-7,7-9", Assignment(Section(5, 7), Section(7, 9))),
            ("2-8,3-7", Assignment(Section(2, 8), Section(3, 7))),
            ("6-6,4-6", Assignment(Section(6, 6), Section(4, 6))),
            ("2-6,4-8", Assignment(Section(2, 6), Section(4, 8))),
        ];

        for case in cases {
            let (str, expected) = case;
            assert_eq!(Assignment::from_str(str).unwrap(), expected);
        }
    }

    #[test]
    fn test_section_contains() {
        let cases = [
            (Section(2, 4), Section(6, 8), false),
            (Section(2, 3), Section(4, 5), false),
            (Section(5, 7), Section(7, 9), false),
            (Section(2, 8), Section(3, 7), true),
            (Section(4, 6), Section(6, 6), true),
            (Section(2, 6), Section(4, 8), false),
        ];

        for case in cases {
            let (a, b, expected) = case;
            assert_eq!(a.contains(&b), expected);
        }
    }

    #[test]
    fn test_has_full_overlap() {
        let cases = [
            (Assignment(Section(2, 4), Section(6, 8)), false),
            (Assignment(Section(2, 3), Section(4, 5)), false),
            (Assignment(Section(5, 7), Section(7, 9)), false),
            (Assignment(Section(2, 8), Section(3, 7)), true),
            (Assignment(Section(6, 6), Section(4, 6)), true),
            (Assignment(Section(2, 6), Section(4, 8)), false),
        ];

        for case in cases {
            let (assignment, expected) = case;
            assert_eq!(assignment.has_full_overlap(), expected);
        }
    }

    #[test]
    fn test_has_overlap() {
        let cases = [
            (Assignment(Section(2, 4), Section(6, 8)), false),
            (Assignment(Section(2, 3), Section(4, 5)), false),
            (Assignment(Section(5, 7), Section(7, 9)), true),
            (Assignment(Section(2, 8), Section(3, 7)), true),
            (Assignment(Section(6, 6), Section(4, 6)), true),
            (Assignment(Section(2, 6), Section(4, 8)), true),
        ];

        for case in cases {
            let (assignment, expected) = case;
            assert_eq!(assignment.has_overlap(), expected);
        }
    }
}
