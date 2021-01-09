use std::fmt::Write;

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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Cell {
    Vacant,
    Occupied(Side),
}
impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Board {
    board: [[Cell; SIZE]; SIZE],
}

impl Board {
    pub const fn new() -> Board {
        let mut board = [[Cell::Vacant; SIZE]; SIZE];
        let c = SIZE / 2 - 1;
        board[c][c] = Cell::Occupied(Side::White);
        board[c + 1][c] = Cell::Occupied(Side::Black);
        board[c][c + 1] = Cell::Occupied(Side::Black);
        board[c + 1][c + 1] = Cell::Occupied(Side::White);
        Board { board }
    }

    fn check_one_dir<I>(&mut self, side: Side, iter: I) -> usize
    where
        I: Iterator<Item = (usize, usize)>,
    {
        let mut count = 0;
        for (c, r) in iter {
            match self.board[r][c] {
                Cell::Vacant => return 0,
                Cell::Occupied(s) if s == side => return count,
                Cell::Occupied(_) => count += 1,
            }
        }
        0
    }

    pub fn put(&mut self, col: usize, row: usize, side: Side) {
        self.board[row][col] = Cell::Occupied(side);
        // col±0 row+1
        let count = self.check_one_dir(side, Repeat(col).zip(row + 1..SIZE));
        for r in row + 1..=row + count {
            self.board[r][col] = Cell::Occupied(side);
        }
        // col±0 row-1
        let count = self.check_one_dir(side, Repeat(col).zip((0..row).rev()));
        for r in row - count..row {
            self.board[r][col] = Cell::Occupied(side);
        }
        // col+1 row±0
        let count = self.check_one_dir(side, (col + 1..SIZE).zip(Repeat(row)));
        for c in col + 1..=col + count {
            self.board[row][c] = Cell::Occupied(side);
        }
        // col-1 row±0
        let count = self.check_one_dir(side, (0..col).rev().zip(Repeat(row)));
        for c in col - count..col {
            self.board[row][c] = Cell::Occupied(side);
        }
        // col+1 row+1
        let count = self.check_one_dir(side, (col + 1..SIZE).zip(row + 1..SIZE));
        for (c, r) in (col + 1..=col + count).zip(row + 1..=row + count) {
            self.board[r][c] = Cell::Occupied(side);
        }
        // col+1 row-1
        let count = self.check_one_dir(side, (col + 1..SIZE).zip((0..row).rev()));
        for (c, r) in (col + 1..=col + count).zip((row - count..row).rev()) {
            self.board[r][c] = Cell::Occupied(side);
        }
        // col-1 row+1
        let count = self.check_one_dir(side, (0..col).rev().zip(row + 1..SIZE));
        for (c, r) in (col - count..col).rev().zip(row + 1..=row + count) {
            self.board[r][c] = Cell::Occupied(side);
        }
        // col-1 row-1
        let count = self.check_one_dir(side, (0..col).rev().zip((0..row).rev()));
        for (c, r) in (col - count..col).zip(row - count..row) {
            self.board[r][c] = Cell::Occupied(side);
        }
    }
}

impl Default for Board {
    fn default() -> Board {
        Board::new()
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.board.iter() {
            for c in row {
                c.fmt(f)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

struct Repeat<T>(T);

impl<T: Copy> Iterator for Repeat<T> {
    type Item = T;
    #[inline]
    fn next(&mut self) -> Option<T> {
        Some(self.0)
    }
}
