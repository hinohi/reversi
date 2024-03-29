use rand::Rng;

use super::{search_exact_with_candidates, Occupied, Position, Search};
use crate::{BitBoard, Candidate, Side};

#[derive(Debug, Clone)]
pub struct RandomSearch<R> {
    rng: R,
}

impl<R> RandomSearch<R> {
    pub const fn new(rng: R) -> RandomSearch<R> {
        RandomSearch { rng }
    }
}

impl<R: Rng> Search for RandomSearch<R> {
    fn search(
        &mut self,
        _board: &BitBoard,
        _occupied: Occupied,
        candidates: &mut Candidate,
        _last_passed: bool,
    ) -> Position {
        let i = self.rng.gen_range(0..candidates.len());
        candidates.nth(i).unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct RandomFullSearch<R> {
    side: Side,
    rng: R,
    full_search_threshold: Occupied,
}

impl<R> RandomFullSearch<R> {
    pub const fn new(side: Side, rng: R, full_search_threshold: Occupied) -> RandomFullSearch<R> {
        RandomFullSearch {
            side,
            rng,
            full_search_threshold,
        }
    }
}

impl<R: Rng> Search for RandomFullSearch<R> {
    fn search(
        &mut self,
        board: &BitBoard,
        occupied: Occupied,
        candidates: &mut Candidate,
        last_passed: bool,
    ) -> Position {
        if occupied < self.full_search_threshold {
            let i = self.rng.gen_range(0..candidates.len());
            candidates.nth(i).unwrap()
        } else {
            search_exact_with_candidates(board, self.side, candidates, last_passed).0
        }
    }
}
