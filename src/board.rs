use std::fmt::Display;

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

pub trait Board: Display + Clone + Eq {
    fn put(&mut self, col: usize, row: usize, side: Side);
}
