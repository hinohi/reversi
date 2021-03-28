use rand::SeedableRng;
use rand_pcg::Mcg128Xsl64;

use reversi::{search::RandomFullSearch, Game, Side};

fn main() {
    let mut rng = Mcg128Xsl64::from_entropy();
    loop {
        for i in 0..=16 {
            for j in 0..=16 {
                let (b, w) = Game::new(
                    RandomFullSearch::new(
                        Side::Black,
                        Mcg128Xsl64::from_rng(&mut rng).unwrap(),
                        64 - i,
                    ),
                    RandomFullSearch::new(
                        Side::White,
                        Mcg128Xsl64::from_rng(&mut rng).unwrap(),
                        64 - j,
                    ),
                )
                .play_game();
                println!("{} {} {} {}", i, j, b, w);
            }
        }
    }
}
