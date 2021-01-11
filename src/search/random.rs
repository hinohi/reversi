use std::marker::PhantomData;

use rand::Rng;

use super::{search_exact_with_candidates, Occupied, Search};
use crate::{Board, Side};

#[derive(Debug, Clone)]
pub struct RandomSearch<B, R> {
    rng: R,
    _phantom: PhantomData<B>,
}

impl<B, R> RandomSearch<B, R> {
    pub const fn new(rng: R) -> RandomSearch<B, R> {
        RandomSearch {
            rng,
            _phantom: PhantomData,
        }
    }
}

impl<B: Board, R: Rng> Search for RandomSearch<B, R> {
    type Board = B;
    fn search(
        &mut self,
        _board: &Self::Board,
        _occupied: Occupied,
        candidates: &[B::Position],
    ) -> usize {
        self.rng.gen_range(0..candidates.len())
    }
}

#[derive(Debug, Clone)]
pub struct RandomFullSearch<B, R> {
    side: Side,
    rng: R,
    full_search_threshold: Occupied,
    _phantom: PhantomData<B>,
}

impl<B, R> RandomFullSearch<B, R> {
    pub const fn new(
        side: Side,
        rng: R,
        full_search_threshold: Occupied,
    ) -> RandomFullSearch<B, R> {
        RandomFullSearch {
            side,
            rng,
            full_search_threshold,
            _phantom: PhantomData,
        }
    }
}

impl<B: Board, R: Rng> Search for RandomFullSearch<B, R> {
    type Board = B;
    fn search(
        &mut self,
        board: &Self::Board,
        occupied: Occupied,
        candidates: &[B::Position],
    ) -> usize {
        if occupied < self.full_search_threshold {
            self.rng.gen_range(0..candidates.len())
        } else {
            search_exact_with_candidates(board, self.side, candidates).0
        }
    }
}
