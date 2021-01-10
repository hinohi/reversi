use std::{fmt::Display, str::FromStr};

pub mod naive;

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

pub trait Board: Display + FromStr + Clone {
    fn put(&mut self, col: usize, row: usize, side: Side);

    fn list_candidates(&self, side: Side) -> Vec<(usize, usize)>;

    /// Calculate the number of black, white
    fn count(&self) -> (Count, Count);
}
