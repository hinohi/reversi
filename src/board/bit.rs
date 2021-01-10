use std::{convert::TryInto, str::FromStr};

use super::{Board, Cell, Count, Side};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct BitBoard {
    black: u64,
    white: u64,
}

impl BitBoard {
    pub const fn new() -> BitBoard {
        BitBoard {
            black: 0x0000000810000000,
            white: 0x0000001008000000,
        }
    }
}

impl Default for BitBoard {
    fn default() -> BitBoard {
        BitBoard::new()
    }
}

pub fn can_put(mine: u64, opp: u64) -> u64 {
    let blank = !(mine | opp);
    let mut pos = 0;
    // 左右
    {
        let watch = opp & 0x7e7e7e7e7e7e7e7e;
        let mut tmp = watch & (mine << 1);
        for _ in 0..5 {
            tmp |= watch & (tmp << 1);
        }
        pos |= blank & (tmp << 1);
        let mut tmp = watch & (mine >> 1);
        for _ in 0..5 {
            tmp |= watch & (tmp >> 1);
        }
        pos |= blank & (tmp >> 1);
    }
    // 上下
    {
        let watch = opp & 0x00FFFFFFFFFFFF00;
        let mut tmp = watch & (mine << 8);
        for _ in 0..5 {
            tmp |= watch & (tmp << 8);
        }
        pos |= blank & (tmp << 8);
        let mut tmp = watch & (mine >> 8);
        for _ in 0..5 {
            tmp |= watch & (tmp >> 8);
        }
        pos |= blank & (tmp >> 8);
    }
    // 斜め
    {
        let watch = opp & 0x007e7e7e7e7e7e00;
        for &shift in [7, 9].iter() {
            let mut tmp = watch & (mine << shift);
            for _ in 0..5 {
                tmp |= watch & (tmp << shift);
            }
            pos |= blank & (tmp << shift);
            let mut tmp = watch & watch & (mine >> shift);
            for _ in 0..5 {
                tmp |= watch & (tmp >> shift);
            }
            pos |= blank & (tmp >> shift);
        }
    }
    pos
}

impl Board for BitBoard {
    fn put(&mut self, col: usize, row: usize, side: Side) {
        let p: u64 = 0x8000000000000000 >> (row * 8 + col);
        let mut rev = 0;
        let (mine, opp) = match side {
            Side::Black => (self.black, self.white),
            Side::White => (self.white, self.black),
        };
        for &(shift, mask) in [
            (8, 0xffffffffffffff00), // 上
            (7, 0x7f7f7f7f7f7f7f00), // 右上
            (1, 0xfefefefefefefefe), // 左
            (9, 0xfefefefefefefe00), // 左上
        ]
        .iter()
        {
            let mut tmp = 0;
            let mut masked = (p << shift) & mask;
            while masked != 0 && (masked & opp) != 0 {
                tmp |= masked;
                masked = (masked << shift) & mask;
            }
            if masked & mine != 0 {
                rev |= tmp;
            }
        }
        for &(shift, mask) in [
            (1, 0x7f7f7f7f7f7f7f7f), // 右
            (9, 0x007f7f7f7f7f7f7f), // 右下
            (8, 0x00ffffffffffffff), // 下
            (7, 0x00fefefefefefefe), // 左下
        ]
        .iter()
        {
            let mut tmp = 0;
            let mut masked = (p >> shift) & mask;
            while masked != 0 && (masked & opp) != 0 {
                tmp |= masked;
                masked = (masked >> shift) & mask;
            }
            if masked & mine != 0 {
                rev |= tmp;
            }
        }
        match side {
            Side::Black => {
                self.black ^= rev | p;
                self.white ^= rev;
            }
            Side::White => {
                self.black ^= rev;
                self.white ^= rev | p;
            }
        }
    }

    fn list_candidates(&self, side: Side) -> Vec<(usize, usize)> {
        let p = match side {
            Side::Black => can_put(self.black, self.white),
            Side::White => can_put(self.white, self.black),
        };
        let mut v = Vec::with_capacity(p.count_ones() as usize);
        let mut mask = 0x8000000000000000_u64;
        for row in 0..8 {
            for col in 0..8 {
                if p & mask != 0 {
                    v.push((col, row));
                }
                mask >>= 1;
            }
        }
        v
    }

    fn count(&self) -> (Count, Count) {
        (
            self.black.count_ones() as Count,
            self.white.count_ones() as Count,
        )
    }
}

impl std::fmt::Display for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        let mut mask = 0x8000000000000000;
        for _ in 0..8 {
            for _ in 0..8 {
                if self.black & mask != 0 {
                    Cell::Occupied(Side::Black).fmt(f)?;
                } else if self.white & mask != 0 {
                    Cell::Occupied(Side::White).fmt(f)?;
                } else {
                    Cell::Vacant.fmt(f)?;
                }
                mask >>= 1
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl FromStr for BitBoard {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let mut p = 0x8000000000000000_u64;
        let mut black = 0;
        let mut white = 0;
        for _ in 0..8 {
            for _ in 0..8 {
                let cell: Cell = chars.next().ok_or(()).map(|c| c.try_into())??;
                match cell {
                    Cell::Occupied(Side::Black) => black ^= p,
                    Cell::Occupied(Side::White) => white ^= p,
                    Cell::Vacant => (),
                }
                p >>= 1;
            }
            match chars.next() {
                Some('\n') | None => (),
                _ => return Err(()),
            }
        }
        Ok(BitBoard { black, white })
    }
}
