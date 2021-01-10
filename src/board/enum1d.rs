use std::{convert::TryInto, str::FromStr};

use super::{Board, Cell, Side, SIZE};

const L: usize = SIZE + 2;
static D: [isize; 8] = {
    let l = L as isize;
    [-1, -l - 1, -l, -l + 1, 1, l + 1, l, l - 1]
};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Enum1dBoard {
    board: Vec<Cell>,
}

impl Enum1dBoard {
    pub fn new() -> Enum1dBoard {
        let mut board = vec![Cell::Vacant; L * L];
        let c = L / 2 - 1;
        board[c * L + c] = Cell::Occupied(Side::White);
        board[c * L + c + 1] = Cell::Occupied(Side::Black);
        board[(c + 1) * L + c] = Cell::Occupied(Side::Black);
        board[(c + 1) * L + c + 1] = Cell::Occupied(Side::White);
        Enum1dBoard { board }
    }

    fn check_one_dir(&self, side: Side, mut p: isize, d: isize) -> usize {
        let mut count = 0;
        loop {
            p += d;
            match self.board[p as usize] {
                Cell::Vacant => break 0,
                Cell::Occupied(s) if s == side => break count,
                Cell::Occupied(_) => count += 1,
            }
        }
    }

    fn put_one_dir(&mut self, side: Side, mut p: isize, d: isize, count: usize) {
        for _ in 1..count {
            p += d;
            self.board[p as usize] = Cell::Occupied(side);
        }
    }

    fn can_put(&self, side: Side, p: isize) -> bool {
        for &d in D.iter() {
            if self.check_one_dir(side, p, d) > 0 {
                return true;
            }
        }
        false
    }
}

impl Board for Enum1dBoard {
    fn put(&mut self, col: usize, row: usize, side: Side) {
        let p = (row + 1) * L + col + 1;
        self.board[p] = Cell::Occupied(side);
        for &d in D.iter() {
            self.put_one_dir(side, p as isize, d, self.check_one_dir(side, p as isize, d));
        }
    }

    fn list_candidates(&self, side: Side) -> Vec<(usize, usize)> {
        let mut v = Vec::new();
        for row in 1..L - 1 {
            for col in 1..L - 1 {
                let p = row * L + col;
                if self.board[p] == Cell::Vacant && self.can_put(side, p as isize) {
                    v.push((col, row));
                }
            }
        }
        v
    }

    fn count(&self) -> (u8, u8) {
        let mut black = 0;
        let mut white = 0;
        for c in self.board.iter() {
            match c {
                Cell::Occupied(Side::Black) => black += 1,
                Cell::Occupied(Side::White) => white += 1,
                Cell::Vacant => (),
            }
        }
        (black, white)
    }
}

impl Default for Enum1dBoard {
    fn default() -> Enum1dBoard {
        Enum1dBoard::new()
    }
}

impl std::fmt::Display for Enum1dBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        for row in 1..=L {
            for col in 1..=L {
                self.board[row * L + col].fmt(f)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl FromStr for Enum1dBoard {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let mut board = vec![Cell::Vacant; L * L];
        for row in 1..L - 1 {
            for col in 1..L - 1 {
                board[row * L + col] = chars.next().ok_or(()).map(|c| c.try_into())??;
            }
            match chars.next() {
                Some('\n') | None => (),
                _ => return Err(()),
            }
        }
        Ok(Enum1dBoard { board })
    }
}
