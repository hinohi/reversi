use std::io::Write;

pub const SIZE: usize = 8;
pub type Count = u8;
pub type Position = u64;

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

fn can_put(mine: u64, opp: u64) -> u64 {
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
            let mut tmp = watch & (mine >> shift);
            for _ in 0..5 {
                tmp |= watch & (tmp >> shift);
            }
            pos |= blank & (tmp >> shift);
        }
    }
    pos
}

impl BitBoard {
    #[inline]
    pub fn mine_opp_keys(&self, side: Side) -> (u64, u64) {
        match side {
            Side::Black => (self.black, self.white),
            Side::White => (self.white, self.black),
        }
    }

    pub fn put(&mut self, side: Side, position: Position) {
        let mut rev = 0;
        let (mine, opp) = self.mine_opp_keys(side);
        for &(shift, mask) in [
            (1, 0xfefefefefefefefe), // 左
            (7, 0x7f7f7f7f7f7f7f00), // 右上
            (8, 0xffffffffffffff00), // 上
            (9, 0xfefefefefefefe00), // 左上
        ]
        .iter()
        {
            let mut tmp = 0;
            let mut masked = (position << shift) & mask;
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
            (7, 0x00fefefefefefefe), // 左下
            (8, 0x00ffffffffffffff), // 下
            (9, 0x007f7f7f7f7f7f7f), // 右下
        ]
        .iter()
        {
            let mut tmp = 0;
            let mut masked = (position >> shift) & mask;
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
                self.black ^= rev | position;
                self.white ^= rev;
            }
            Side::White => {
                self.black ^= rev;
                self.white ^= rev | position;
            }
        }
    }

    #[inline]
    pub fn candidates(&self, side: Side) -> Candidate {
        let (mine, opp) = self.mine_opp_keys(side);
        let pos = can_put(mine, opp);
        Candidate { pos }
    }

    #[inline]
    pub fn count(&self) -> (Count, Count) {
        (
            self.black.count_ones() as Count,
            self.white.count_ones() as Count,
        )
    }

    pub fn col_row(position: Position) -> (usize, usize) {
        let position = position.leading_zeros() as usize;
        (position % 8, position / 8)
    }

    pub fn position(col: usize, row: usize) -> Position {
        0x8000000000000000 >> (row * 8 + col)
    }
}

#[derive(Debug)]
pub struct Candidate {
    pos: u64,
}

impl Iterator for Candidate {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        if self.pos == 0 {
            None
        } else {
            let p = 0x8000000000000000 >> self.pos.leading_zeros();
            self.pos -= p;
            Some(p)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let c = self.pos.count_ones() as usize;
        (c, Some(c))
    }
}

impl ExactSizeIterator for Candidate {
    fn len(&self) -> usize {
        self.pos.count_ones() as usize
    }
}

impl BitBoard {
    pub fn format<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        let mut mask = 0x8000000000000000;
        for _ in 0..8 {
            for _ in 0..8 {
                if self.black & mask != 0 {
                    write!(writer, "●")?;
                } else if self.white & mask != 0 {
                    write!(writer, "○")?;
                } else {
                    write!(writer, "_")?;
                }
                mask >>= 1
            }
            writeln!(writer)?;
        }
        Ok(())
    }
}

impl BitBoard {
    pub fn symmetry(&self, flag: u32) -> BitBoard {
        let mut sym = self.clone();
        if flag & 1 != 0 {
            sym.black = horizontal_mirror(sym.black);
            sym.white = horizontal_mirror(sym.white);
        }
        if flag & 2 != 0 {
            sym.black = vertical_mirror(sym.black);
            sym.white = vertical_mirror(sym.white);
        }
        if flag & 4 != 0 {
            sym.black = transpose(sym.black);
            sym.white = transpose(sym.white);
        }
        sym
    }

    pub fn unique(&self) -> BitBoard {
        let mut u = self.clone();
        for f in 1..8 {
            let sym = self.symmetry(f);
            if (sym.black, sym.white) < (u.black, u.white) {
                u = sym;
            }
        }
        u
    }
}

fn horizontal_mirror(mut b: u64) -> u64 {
    b = ((b >> 1) & 0x5555555555555555) | ((b << 1) & 0xAAAAAAAAAAAAAAAA);
    b = ((b >> 2) & 0x3333333333333333) | ((b << 2) & 0xCCCCCCCCCCCCCCCC);
    b = ((b >> 4) & 0x0F0F0F0F0F0F0F0F) | ((b << 4) & 0xF0F0F0F0F0F0F0F0);
    b
}

fn vertical_mirror(mut b: u64) -> u64 {
    b = ((b >> 8) & 0x00FF00FF00FF00FF) | ((b << 8) & 0xFF00FF00FF00FF00);
    b = ((b >> 16) & 0x0000FFFF0000FFFF) | ((b << 16) & 0xFFFF0000FFFF0000);
    b = ((b >> 32) & 0x00000000FFFFFFFF) | ((b << 32) & 0xFFFFFFFF00000000);
    b
}

fn transpose(mut b: u64) -> u64 {
    let mut t = (b ^ (b >> 7)) & 0x00aa00aa00aa00aa;
    b = b ^ t ^ (t << 7);
    t = (b ^ (b >> 14)) & 0x0000cccc0000cccc;
    b = b ^ t ^ (t << 14);
    t = (b ^ (b >> 28)) & 0x00000000f0f0f0f0;
    b = b ^ t ^ (t << 28);
    b
}
