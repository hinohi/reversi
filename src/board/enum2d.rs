use std::{convert::TryInto, str::FromStr};

use super::{Board, Cell, Side, SIZE};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Enum2dBoard {
    board: [[Cell; SIZE]; SIZE],
}

impl Enum2dBoard {
    pub const fn new() -> Enum2dBoard {
        let mut board = [[Cell::Vacant; SIZE]; SIZE];
        let c = SIZE / 2 - 1;
        board[c][c] = Cell::Occupied(Side::White);
        board[c + 1][c] = Cell::Occupied(Side::Black);
        board[c][c + 1] = Cell::Occupied(Side::Black);
        board[c + 1][c + 1] = Cell::Occupied(Side::White);
        Enum2dBoard { board }
    }

    fn check_one_dir<I>(&self, side: Side, iter: I) -> usize
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

    fn can_put(&self, col: usize, row: usize, side: Side) -> bool {
        if self.check_one_dir(side, (col + 1..SIZE).zip(Repeat(row))) > 0 {
            return true;
        }
        if self.check_one_dir(side, (0..col).rev().zip(Repeat(row))) > 0 {
            return true;
        }
        if self.check_one_dir(side, Repeat(col).zip(row + 1..SIZE)) > 0 {
            return true;
        }
        if self.check_one_dir(side, Repeat(col).zip((0..row).rev())) > 0 {
            return true;
        }
        if self.check_one_dir(side, (col + 1..SIZE).zip(row + 1..SIZE)) > 0 {
            return true;
        }
        if self.check_one_dir(side, (col + 1..SIZE).zip((0..row).rev())) > 0 {
            return true;
        }
        if self.check_one_dir(side, (0..col).rev().zip(row + 1..SIZE)) > 0 {
            return true;
        }
        if self.check_one_dir(side, (0..col).rev().zip((0..row).rev())) > 0 {
            return true;
        }
        false
    }
}

impl Board for Enum2dBoard {
    type Position = (usize, usize);
    fn put(&mut self, side: Side, position: Self::Position) {
        let (col, row) = position;
        self.board[row][col] = Cell::Occupied(side);
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

    fn list_candidates(&self, side: Side) -> Vec<Self::Position> {
        let mut v = Vec::new();
        for (i, row) in self.board.iter().enumerate() {
            for (j, &c) in row.iter().enumerate() {
                if c == Cell::Vacant && self.can_put(j, i, side) {
                    v.push((j, i))
                }
            }
        }
        v
    }

    fn count(&self) -> (u8, u8) {
        let mut black = 0;
        let mut white = 0;
        for row in self.board.iter() {
            for c in row.iter() {
                match c {
                    Cell::Occupied(Side::Black) => black += 1,
                    Cell::Occupied(Side::White) => white += 1,
                    Cell::Vacant => (),
                }
            }
        }
        (black, white)
    }

    fn col_row(position: Self::Position) -> (usize, usize) {
        position
    }

    fn position(col: usize, row: usize) -> Self::Position {
        (col, row)
    }
}

impl Default for Enum2dBoard {
    fn default() -> Enum2dBoard {
        Enum2dBoard::new()
    }
}

impl std::fmt::Display for Enum2dBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        for row in self.board.iter() {
            for c in row {
                c.fmt(f)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl FromStr for Enum2dBoard {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let mut board = [[Cell::Vacant; SIZE]; SIZE];
        for row in board.iter_mut() {
            for cell in row.iter_mut() {
                *cell = chars.next().ok_or(()).map(|c| c.try_into())??;
            }
            match chars.next() {
                Some('\n') | None => (),
                _ => return Err(()),
            }
        }
        Ok(Enum2dBoard { board })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str() {
        let mut expect = Enum2dBoard::default();
        expect.put(Side::Black, (2, 3));
        let s = r"________
________
__*_*___
__●●●___
__*●○___
________
________
________";
        assert_eq!(expect, s.parse().unwrap());
    }
}
