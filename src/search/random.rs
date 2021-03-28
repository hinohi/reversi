use rand::Rng;

use super::{search_exact_with_candidates, Occupied, Search};
use crate::{BitBoard, Side};

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
    fn search(&mut self, _board: &BitBoard, _occupied: Occupied, candidates: &[u8]) -> usize {
        self.rng.gen_range(0..candidates.len())
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
    fn search(&mut self, board: &BitBoard, occupied: Occupied, candidates: &[u8]) -> usize {
        if occupied < self.full_search_threshold {
            self.rng.gen_range(0..candidates.len())
        } else {
            search_exact_with_candidates(board, self.side, candidates).0
        }
    }
}
