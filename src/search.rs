use crate::{Board, Count, Side, SIZE};

pub type Turn = u8;

pub trait Score: Copy + PartialOrd {
    const MIN: Self;
    const MAX: Self;
    fn flip(&self) -> Self;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct CountWithTurn {
    pub mine: Count,
    pub opp: Count,
    pub turn: Turn,
}

impl CountWithTurn {
    pub const fn new(mine: Count, opp: Count, turn: Turn) -> CountWithTurn {
        CountWithTurn { mine, opp, turn }
    }

    pub const fn with_side(side: Side, black: Count, white: Count, turn: Turn) -> CountWithTurn {
        match side {
            Side::Black => CountWithTurn::new(black, white, turn),
            Side::White => CountWithTurn::new(white, black, turn),
        }
    }
}

impl PartialOrd for CountWithTurn {
    fn partial_cmp(&self, other: &CountWithTurn) -> Option<std::cmp::Ordering> {
        use std::cmp::{Ordering::*, Reverse};
        if self.opp == 0 {
            if other.opp == 0 {
                // 両者とも全滅させてるなら
                // 1. 早い方が良い
                // 2. 自分が多い方が良い (これは turn が同じなら同じなのが普通)
                (Reverse(self.turn), self.mine).partial_cmp(&(Reverse(other.turn), other.mine))
            } else {
                // 全滅の方が良い
                Some(Greater)
            }
        } else {
            if other.opp == 0 {
                // 全滅の方が良い
                Some(Less)
            } else {
                // 割合ではなく絶対値
                let a = (self.mine, Reverse(self.opp), Reverse(self.turn));
                let b = (other.mine, Reverse(other.opp), Reverse(other.turn));
                a.partial_cmp(&b)
            }
        }
    }
}

impl Score for CountWithTurn {
    const MIN: CountWithTurn = CountWithTurn::new(0, (SIZE * SIZE) as Count, 0);
    const MAX: CountWithTurn = CountWithTurn::new((SIZE * SIZE) as Count, 0, 0);
    fn flip(&self) -> CountWithTurn {
        CountWithTurn::new(self.opp, self.mine, self.turn)
    }
}

pub fn search_exact<B: Board>(board: &B, side: Side) -> CountWithTurn {
    exact_inner(
        board,
        side,
        false,
        0,
        CountWithTurn::MIN,
        CountWithTurn::MAX,
    )
}

fn exact_inner<B: Board>(
    board: &B,
    side: Side,
    passed: bool,
    turn: u8,
    mut alpha: CountWithTurn,
    beta: CountWithTurn,
) -> CountWithTurn {
    let candidates = board.list_candidates(side);
    if candidates.is_empty() {
        return if passed {
            let (black, white) = board.count();
            CountWithTurn::with_side(side, black, white, turn)
        } else {
            exact_inner(board, side.flip(), true, turn, beta.flip(), alpha.flip()).flip()
        };
    }
    for (col, row) in candidates {
        let mut board = board.clone();
        board.put(col, row, side);
        let a = exact_inner(
            &board,
            side.flip(),
            false,
            turn + 1,
            beta.flip(),
            alpha.flip(),
        )
        .flip();
        if a > alpha {
            alpha = a;
        }
        if alpha >= beta {
            break;
        }
    }
    alpha
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_with_turn_ord() {
        assert!(CountWithTurn::new(40, 24, 10) > CountWithTurn::new(39, 24, 10));
        assert!(CountWithTurn::new(40, 24, 10) < CountWithTurn::new(40, 23, 10));
        assert!(CountWithTurn::new(60, 4, 3) < CountWithTurn::new(10, 0, 3));
        assert!(CountWithTurn::new(10, 0, 10) > CountWithTurn::new(11, 0, 11));
        assert!(CountWithTurn::new(10, 0, 10) < CountWithTurn::new(11, 0, 10));
    }
}
