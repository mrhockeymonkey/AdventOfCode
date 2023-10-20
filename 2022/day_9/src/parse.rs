use std::fmt::{Debug, Formatter};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::space1;
use nom::combinator::{map, value};
use nom::IResult;
use nom::sequence::{preceded, tuple};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct GridPos {
    pub(crate) x: i32,
    pub(crate) y: i32
}

impl Debug for GridPos {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl std::ops::Add for GridPos {
    type Output = GridPos;

    fn add(self, rhs: Self) -> Self::Output {
        GridPos {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::AddAssign for GridPos {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl std::ops::Sub for GridPos {
    type Output = GridPos;

    fn sub(self, rhs: Self) -> Self::Output {
        GridPos {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub(crate) fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            value(Direction::Up, tag("U")),
            value(Direction::Down, tag("D")),
            value(Direction::Left, tag("L")),
            value(Direction::Right, tag("R")),
        ))(input)
    }
    
    pub(crate) fn delta(self) -> GridPos {
        match self { 
            Direction::Up => GridPos{x: 0, y: -1}, // surely this is wrong
            Direction::Down => GridPos{x: 0, y: 1},
            Direction::Left => GridPos{x: -1, y: 0},
            Direction::Right => GridPos{x: 1, y: 0},
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct Instruction {
    pub(crate) dir: Direction,
    pub(crate) dist: u32,
}

impl Instruction {
    pub(crate) fn parse(input: &str) -> IResult<&str, Self> {
        map(
            tuple((
                Direction::parse,
                space1,
                nom::character::complete::u32,
            )),
            |(dir, _, dist)| Self { dir, dist },
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::parse::{Direction, GridPos, Instruction};

    #[test]
    fn add_assign() {
        let mut a = GridPos{x: 2, y: 3};
        let b = GridPos{x: 3, y: 4};
        
        a += b;
        
        assert_eq!(a.x, 5);
        assert_eq!(a.y, 7);
    }
    
    #[test]
    fn parse_instruction() {
        let ins = "U 2";
        let result = Instruction::parse(ins).unwrap().1;
        assert_eq!(result.dir, Direction::Up);
        assert_eq!(result.dist, 2);
    }
}
