mod random;

pub use self::random::*;
use crate::{BitBoard, Candidate, Count, Position, Side, SIZE};

pub type Occupied = u8;

pub trait Search {
    fn search(
        &mut self,
        board: &BitBoard,
        occupied: Occupied,
        candidates: &mut Candidate,
        last_passed: bool,
    ) -> Position;
}

pub type Turn = u8;

pub trait Score: Copy + PartialOrd {
    const MIN: Self;
    const MAX: Self;
    fn flip(&self) -> Self;
}

/// Count & Turn
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct CountTurn {
    pub mine: Count,
    pub opp: Count,
    pub turn: Turn,
}

impl CountTurn {
    pub const fn new(mine: Count, opp: Count, turn: Turn) -> CountTurn {
        CountTurn { mine, opp, turn }
    }

    pub const fn with_side(side: Side, black: Count, white: Count, turn: Turn) -> CountTurn {
        match side {
            Side::Black => CountTurn::new(black, white, turn),
            Side::White => CountTurn::new(white, black, turn),
        }
    }
}

impl Ord for CountTurn {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::{Ordering::*, Reverse as R};
        match (self.mine, self.opp, other.mine, other.opp) {
            // 両者とも全滅させてるなら
            // 1. 早い方が良い
            // 2. 自分が多い方が良い (これは turn が同じなら同じなのが普通)
            (sm, 0, om, 0) => (R(self.turn), sm).cmp(&(R(other.turn), om)),
            // その逆
            (0, so, 0, oo) => (self.turn, R(so)).cmp(&(self.turn, R(oo))),
            // 全滅の方が良い
            (_, 0, _, _) => Greater,
            (_, _, _, 0) => Less,
            // 割合ではなく絶対値で比較
            (sm, so, om, oo) => {
                (sm as i8 - so as i8, R(self.turn)).cmp(&(om as i8 - oo as i8, R(other.turn)))
            }
        }
    }
}

impl PartialOrd for CountTurn {
    fn partial_cmp(&self, other: &CountTurn) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Score for CountTurn {
    const MIN: CountTurn = CountTurn::new(0, (SIZE * SIZE) as Count, 0);
    const MAX: CountTurn = CountTurn::new((SIZE * SIZE) as Count, 0, 0);
    fn flip(&self) -> CountTurn {
        CountTurn::new(self.opp, self.mine, self.turn)
    }
}

pub fn search_exact(board: &BitBoard, side: Side, last_passed: bool) -> CountTurn {
    exact_inner(board, side, last_passed, 0, CountTurn::MIN, CountTurn::MAX)
}

pub fn search_exact_with_candidates(
    board: &BitBoard,
    side: Side,
    candidates: &mut Candidate,
    last_passed: bool,
) -> (Position, CountTurn) {
    let mut alpha = CountTurn::MIN;
    let mut best = 0;
    for pos in candidates {
        let mut board = board.clone();
        board.put(side, pos);
        let a = exact_inner(
            &board,
            side.flip(),
            last_passed,
            1,
            CountTurn::MIN,
            alpha.flip(),
        )
        .flip();
        if a > alpha {
            alpha = a;
            best = pos;
        }
    }
    (best, alpha)
}

fn exact_inner(
    board: &BitBoard,
    side: Side,
    passed: bool,
    turn: u8,
    mut alpha: CountTurn,
    beta: CountTurn,
) -> CountTurn {
    let candidates = board.candidates(side);
    if candidates.size_hint().0 == 0 {
        return if passed {
            let (black, white) = board.count();
            CountTurn::with_side(side, black, white, turn)
        } else {
            exact_inner(board, side.flip(), true, turn, beta.flip(), alpha.flip()).flip()
        };
    }
    for pos in candidates {
        let mut board = board.clone();
        board.put(side, pos);
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
    fn count_with_turn_less_or_greater() {
        // a < b
        let cases = &[
            (CountTurn::new(39, 24, 10), CountTurn::new(40, 24, 10)),
            (CountTurn::new(40, 24, 10), CountTurn::new(40, 23, 10)),
            (CountTurn::new(60, 4, 3), CountTurn::new(10, 0, 3)),
            (CountTurn::new(11, 0, 11), CountTurn::new(10, 0, 10)),
            (CountTurn::new(10, 0, 10), CountTurn::new(11, 0, 10)),
        ];
        for (a, b) in cases {
            assert!(a < b);
            assert!(b > a);
        }
    }
}
