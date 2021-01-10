use std::{convert::TryFrom, fmt::Display, str::FromStr};

pub mod enum1d;
pub mod enum2d;

pub const SIZE: usize = 8;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Side {
    Black,
    White,
}

impl Side {
    /// Flip the side
    ///
    /// ```rust
    /// # use reversi::Side;
    /// assert_eq!(Side::Black.flip(), Side::White);
    /// assert_eq!(Side::White.flip(), Side::Black);
    /// ```
    #[inline]
    pub const fn flip(self) -> Side {
        use Side::*;
        match self {
            Black => White,
            White => Black,
        }
    }
}

pub type Count = u8;

pub trait Board: Display + FromStr + Clone + Default {
    fn put(&mut self, col: usize, row: usize, side: Side);

    fn list_candidates(&self, side: Side) -> Vec<(usize, usize)>;

    /// Calculate the number of black, white
    fn count(&self) -> (Count, Count);
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Cell {
    Vacant,
    Occupied(Side),
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        match self {
            Cell::Vacant => f.write_char('_'),
            Cell::Occupied(Side::Black) => f.write_char('●'),
            Cell::Occupied(Side::White) => f.write_char('○'),
        }
    }
}

impl Default for Cell {
    fn default() -> Cell {
        Cell::Vacant
    }
}

impl TryFrom<char> for Cell {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '_' | '*' => Ok(Cell::Vacant),
            '●' => Ok(Cell::Occupied(Side::Black)),
            '○' => Ok(Cell::Occupied(Side::White)),
            _ => Err(()),
        }
    }
}
