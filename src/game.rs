use crate::{
    board::{BitBoard, Count, Side},
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
    board: BitBoard,
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
            board: BitBoard::new(),
            black_searcher,
            white_searcher,
        }
    }

    fn game_set(&self) -> ActionResult {
        let (b, w) = self.board.count();
        ActionResult::GameSet(b, w)
    }

    pub fn play_one_turn(&mut self) -> ActionResult {
        if self.occupied == 64 {
            return self.game_set();
        }
        let mut candidates = self.board.candidates(self.side);
        // これは暗黙の仕様すぎる気もする
        if candidates.size_hint().0 == 0 {
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
                let position =
                    self.black_searcher
                        .search(&self.board, self.occupied, &mut candidates);
                self.board.put(self.side, position);
                let (col, row) = BitBoard::col_row(position);
                (Side::Black, col, row)
            }
            Side::White => {
                let position =
                    self.white_searcher
                        .search(&self.board, self.occupied, &mut candidates);
                self.board.put(self.side, position);
                let (col, row) = BitBoard::col_row(position);
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
