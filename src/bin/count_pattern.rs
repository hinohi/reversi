use reversi::{bit::BitBoard, Board, Side};
use std::collections::HashSet;

fn main() {
    let mut unique = HashSet::new();
    let mut direct = HashSet::new();
    unique.insert(BitBoard::new());
    direct.insert(BitBoard::new());
    let mut side = Side::Black;
    for turn in 1..=11 {
        let mut next_unique = HashSet::with_capacity(unique.len());
        let mut next_direct = HashSet::with_capacity(direct.len());
        for b in unique.iter() {
            for p in b.list_candidates(side) {
                let mut b = b.clone();
                b.put(side, p);
                next_unique.insert(b.unique());
            }
        }
        for b in direct.iter() {
            for p in b.list_candidates(side) {
                let mut b = b.clone();
                b.put(side, p);
                next_direct.insert(b);
            }
        }
        println!(
            "{}\n  {}->{}\n  {}->{}",
            turn,
            unique.len(),
            next_unique.len(),
            direct.len(),
            next_direct.len(),
        );
        side = side.flip();
        unique = next_unique;
        direct = next_direct;
    }
}
