use std::{cmp::Ordering, fmt::Display};

use exhaust::Exhaust;
use grid::Grid;

use crate::gridtools::IsValidIndex;

use super::coordinate::{Coordinate, CoordinateOffset};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Exhaust)]
pub enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Direction {
    pub fn to_delta(&self) -> CoordinateOffset {
        match self {
            Self::Left => (0, -1),
            Self::Right => (0, 1),
            Self::Up => (-1, 0),
            Self::Down => (1, 0),
        }
        .into()
    }

    pub fn from_coords(a: Coordinate, b: Coordinate) -> Self {
        assert!(
            (a.x == b.x) ^ (a.y == b.y),
            "coordinates must be unique and lie on a cardinal line {},{}",
            a,
            b
        );
        match (a.x.cmp(&b.x), a.y.cmp(&b.y)) {
            (_, Ordering::Less) => Self::Right,
            (_, Ordering::Greater) => Self::Left,
            (Ordering::Less, _) => Self::Down,
            (Ordering::Greater, _) => Self::Up,
            _ => unreachable!(),
        }
    }

    pub fn from_char(c: u8) -> Self {
        match c {
            b'<' => Self::Left,
            b'>' => Self::Right,
            b'^' => Self::Up,
            b'v' => Self::Down,
            _ => unreachable!(),
        }
    }

    pub fn to_char(&self) -> u8 {
        match self {
            Self::Left => b'<',
            Self::Right => b'>',
            Self::Up => b'^',
            Self::Down => b'v',
        }
    }

    pub fn opposite(&self) -> Self {
        match self {
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Up => Self::Down,
            Self::Right => Self::Left,
        }
    }

    pub fn cw(&self) -> Self {
        match self {
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Up => Self::Right,
            Self::Right => Self::Down,
        }
    }

    pub fn cw_turns(&self, other: Self) -> usize {
        (0..4)
            .find(|&n| {
                let mut dir = *self;
                for _ in 0..n {
                    dir = dir.cw();
                }
                dir == other
            })
            .unwrap()
    }

    pub fn move_dir(&self, pos: Coordinate) -> CoordinateOffset {
        let d = self.to_delta();
        (pos.x as isize + d.x, pos.y as isize + d.y).into()
    }

    pub fn try_move_dir_on_grid<T>(&self, pos: Coordinate, g: &Grid<T>) -> Option<Coordinate> {
        let next_pos = self.move_dir(pos);
        if g.is_valid_index(next_pos) {
            Some(next_pos.into())
        } else {
            None
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Down => "v",
                Self::Left => "<",
                Self::Right => ">",
                Self::Up => "^",
            }
        )
    }
}
