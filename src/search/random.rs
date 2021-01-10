use std::marker::PhantomData;

use rand::Rng;

use super::{Occupied, Search};
use crate::Board;

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
        candidates: &[(usize, usize)],
    ) -> usize {
        self.rng.gen_range(0..candidates.len())
    }
}
