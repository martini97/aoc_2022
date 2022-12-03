use std::str::FromStr;

use crate::day_02_a::Choice;

#[derive(Debug, PartialEq, Clone)]
pub enum MatchResult {
    Loose = 0,
    Draw = 1,
    Win = 2,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Match {
    other: Choice,
    result: MatchResult,
}

impl FromStr for MatchResult {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(MatchResult::Loose),
            "Y" => Ok(MatchResult::Draw),
            "Z" => Ok(MatchResult::Win),
            _ => Err(()),
        }
    }
}

impl FromStr for Match {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (other, result) = s.trim().split_once(" ").unwrap();
        Ok(Match {
            result: MatchResult::from_str(result).unwrap(),
            other: Choice::from_str(other).unwrap(),
        })
    }
}

impl MatchResult {
    pub fn to_str(&self) -> &str {
        match self {
            Self::Win => "win",
            Self::Draw => "draw",
            Self::Loose => "loose",
        }
    }
}

impl Match {
    pub fn score(&self) -> i16 {
        let result = self.result.to_owned() as i16;
        let other = self.other.to_owned() as i16;
        return 3 * result + (result + 1 + other) % 3 + 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn match_result_parse_from_str() {
        assert_eq!(MatchResult::from_str("X").unwrap(), MatchResult::Loose);
        assert_eq!(MatchResult::from_str("Y").unwrap(), MatchResult::Draw);
        assert_eq!(MatchResult::from_str("Z").unwrap(), MatchResult::Win);
        assert!(MatchResult::from_str("invalid").is_err());
    }

    #[test]
    fn match_parse_from_str() {
        assert_eq!(
            Match::from_str("A Y").unwrap(),
            Match {
                result: MatchResult::Draw,
                other: Choice::Rock,
            }
        );
        assert_eq!(
            Match::from_str("B X").unwrap(),
            Match {
                result: MatchResult::Loose,
                other: Choice::Paper
            }
        );
        assert_eq!(
            Match::from_str("C Z").unwrap(),
            Match {
                result: MatchResult::Win,
                other: Choice::Scissor
            }
        );
    }

    #[test]
    fn match_score() {
        assert_eq!(Match::from_str("A Y").unwrap().score(), 4);
        assert_eq!(Match::from_str("B X").unwrap().score(), 1);
        assert_eq!(Match::from_str("C Z").unwrap().score(), 7);
    }
}
