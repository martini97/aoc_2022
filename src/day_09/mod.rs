use std::{
    ops::{Add, AddAssign, Sub},
    str::FromStr,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Point {
    x: isize,
    y: isize,
}

pub struct Rope<const N: usize> {
    knots: [Point; N],
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        return Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        return Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        };
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &*s.to_lowercase() {
            "r" => Ok(Self { x: 1, y: 0 }),
            "u" => Ok(Self { x: 0, y: 1 }),
            "l" => Ok(Self { x: -1, y: 0 }),
            "d" => Ok(Self { x: 0, y: -1 }),
            _ => Err(()),
        }
    }
}

impl Point {
    pub fn is_adjacent(&self, other: Self) -> bool {
        return self.x.abs_diff(other.x) <= 1 && self.y.abs_diff(other.y) <= 1;
    }

    pub fn get_vector(&self, other: Self) -> Self {
        let vector = other - *self;
        return Self {
            x: vector.x / vector.x.abs().max(1),
            y: vector.y / vector.y.abs().max(1),
        };
    }
}

impl<const N: usize> Rope<N> {
    pub fn new() -> Self {
        return Self {
            knots: [Point { x: 0, y: 0 }; N],
        };
    }

    pub fn drag(&mut self, direction: Point) {
        self.knots[0] += direction;

        for i in 1..N {
            let curr = self.knots[i];
            let prev = self.knots[i - 1];
            if curr.is_adjacent(prev) {
                continue;
            }

            self.knots[i] += curr.get_vector(prev);
        }
    }

    pub fn tail(&self) -> Point {
        return self.knots[N - 1];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_add() {
        let test_cases = [
            ((0, 0), (1, 1), (1, 1)),
            ((0, 0), (-1, 1), (-1, 1)),
            ((5, 2), (-1, 1), (4, 3)),
        ];

        for (a, b, expected) in test_cases {
            let a = Point { x: a.0, y: a.1 };
            let b = Point { x: b.0, y: b.1 };
            let expected = Point {
                x: expected.0,
                y: expected.1,
            };

            assert_eq!(a + b, expected);
        }
    }

    #[test]
    fn test_new_rope() {
        let rope = Rope::<2>::new();
        assert_eq!(rope.knots.len(), 2);
        for knot in rope.knots {
            assert_eq!(knot, Point { x: 0, y: 0 });
        }
    }

    #[test]
    fn test_drap_rop() {
        let mut rope = Rope::<3>::new();
        let right = Point { x: 1, y: 0 };
        let up = Point { x: 0, y: 1 };

        rope.drag(right);

        assert_eq!(rope.knots[0], Point { x: 1, y: 0 });
        assert_eq!(rope.knots[1], Point { x: 0, y: 0 });
        assert_eq!(rope.knots[2], Point { x: 0, y: 0 });

        rope.drag(right);

        assert_eq!(rope.knots[0], Point { x: 2, y: 0 });
        assert_eq!(rope.knots[1], Point { x: 1, y: 0 });
        assert_eq!(rope.knots[2], Point { x: 0, y: 0 });

        rope.drag(right);

        assert_eq!(rope.knots[0], Point { x: 3, y: 0 });
        assert_eq!(rope.knots[1], Point { x: 2, y: 0 });
        assert_eq!(rope.knots[2], Point { x: 1, y: 0 });

        rope.drag(up);

        assert_eq!(rope.knots[0], Point { x: 3, y: 1 });
        assert_eq!(rope.knots[1], Point { x: 2, y: 0 });
        assert_eq!(rope.knots[2], Point { x: 1, y: 0 });

        rope.drag(up);

        assert_eq!(rope.knots[0], Point { x: 3, y: 2 });
        assert_eq!(rope.knots[1], Point { x: 3, y: 1 });
        assert_eq!(rope.knots[2], Point { x: 2, y: 1 });
    }
}
