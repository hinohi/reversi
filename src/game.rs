use crate::{
    board::{Board, Count, Side},
    search::{Occupied, Search},
};
use std::fmt::Debug;

#[derive(Debug)]
pub struct Game<B, W>
where
    B: Search,
    W: Search,
{
    side: Side,
    occupied: Occupied,
    last_passed: bool,
    black_board: B::Board,
    white_board: W::Board,
    black_searcher: B,
    white_searcher: W,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ActionResult {
    GameSet(Count, Count),
    Pass(Side),
    Put(Side, usize, usize),
}

impl<B, W> Game<B, W>
where
    B: Search,
    W: Search,
{
    pub fn new(black_searcher: B, white_searcher: W) -> Game<B, W> {
        Game {
            side: Side::Black,
            occupied: 4,
            last_passed: false,
            black_board: B::Board::default(),
            white_board: W::Board::default(),
            black_searcher,
            white_searcher,
        }
    }

    fn game_set(&self) -> ActionResult {
        let (b, w) = self.black_board.count();
        ActionResult::GameSet(b, w)
    }

    pub fn play_one_turn(&mut self) -> ActionResult {
        if self.occupied == 64 {
            return self.game_set();
        }
        let b_candidates = self.black_board.list_candidates(self.side);
        if b_candidates.is_empty() {
            return if self.last_passed {
                self.game_set()
            } else {
                let action = ActionResult::Pass(self.side);
                self.side = self.side.flip();
                self.last_passed = true;
                action
            };
        }
        let (side, col, row) = match self.side {
            Side::Black => {
                let choice =
                    self.black_searcher
                        .search(&self.black_board, self.occupied, &b_candidates);
                let pos = b_candidates[choice];
                self.black_board.put(self.side, pos);
                let (col, row) = B::Board::col_row(pos);
                self.white_board
                    .put(self.side, W::Board::position(col, row));
                (Side::Black, col, row)
            }
            Side::White => {
                let w_candidates = self.white_board.list_candidates(self.side);
                let choice =
                    self.white_searcher
                        .search(&self.white_board, self.occupied, &w_candidates);
                let pos = w_candidates[choice];
                self.white_board.put(self.side, pos);
                let (col, row) = W::Board::col_row(pos);
                self.black_board
                    .put(self.side, B::Board::position(col, row));
                (Side::White, col, row)
            }
        };
        self.side = self.side.flip();
        self.last_passed = false;
        self.occupied += 1;
        ActionResult::Put(side, col, row)
    }

    pub fn play_game(&mut self) -> (Count, Count) {
        loop {
            if let ActionResult::GameSet(b, w) = self.play_one_turn() {
                break (b, w);
            }
        }
    }
}
