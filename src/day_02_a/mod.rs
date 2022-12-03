use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Choice {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Match {
    me: Choice,
    other: Choice,
}

impl FromStr for Choice {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Choice::Rock),
            "B" | "Y" => Ok(Choice::Paper),
            "C" | "Z" => Ok(Choice::Scissor),
            _ => Err(()),
        }
    }
}

impl Choice {
    pub fn from_i32(x: i32) -> Result<Self, ()> {
        match x.rem_euclid(3) {
            0 => Ok(Choice::Scissor),
            1 => Ok(Choice::Rock),
            2 => Ok(Choice::Paper),
            _ => Err(()),
        }
    }
}

impl FromStr for Match {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (other, me) = s.trim().split_once(" ").unwrap();
        Ok(Match {
            me: Choice::from_str(me).unwrap(),
            other: Choice::from_str(other).unwrap(),
        })
    }
}

impl Match {
    pub fn score(&self) -> i16 {
        let me = self.me.to_owned() as i16;
        let other = self.other.to_owned() as i16;
        return me + 3 * ((1 + me - other).rem_euclid(3));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn choice_parse_from_str() {
        assert_eq!(Choice::from_str("A").unwrap(), Choice::Rock);
        assert_eq!(Choice::from_str("X").unwrap(), Choice::Rock);

        assert_eq!(Choice::from_str("B").unwrap(), Choice::Paper);
        assert_eq!(Choice::from_str("Y").unwrap(), Choice::Paper);

        assert_eq!(Choice::from_str("C").unwrap(), Choice::Scissor);
        assert_eq!(Choice::from_str("Z").unwrap(), Choice::Scissor);

        assert_eq!(Choice::from_str("bad strig").is_err(), true);
    }

    #[test]
    fn choice_parse_from_i32() {
        assert_eq!(Choice::from_i32(1).unwrap(), Choice::Rock);
        assert_eq!(Choice::from_i32(2).unwrap(), Choice::Paper);
        assert_eq!(Choice::from_i32(3).unwrap(), Choice::Scissor);
    }

    #[test]
    fn match_parse_from_str() {
        assert_eq!(
            Match::from_str("A Y").unwrap(),
            Match {
                me: Choice::Paper,
                other: Choice::Rock
            }
        );
        assert_eq!(
            Match::from_str("B X").unwrap(),
            Match {
                me: Choice::Rock,
                other: Choice::Paper
            }
        );
        assert_eq!(
            Match::from_str("C Z").unwrap(),
            Match {
                me: Choice::Scissor,
                other: Choice::Scissor
            }
        );
    }

    #[test]
    fn match_score() {
        assert_eq!(Match::from_str("A Y").unwrap().score(), 8);
        assert_eq!(Match::from_str("B X").unwrap().score(), 1);
        assert_eq!(Match::from_str("C Z").unwrap().score(), 6);
    }
}
