use rand::SeedableRng;
use rand_pcg::Mcg128Xsl64;

use reversi::{bit::BitBoard, search::RandomFullSearch, Game, Side};

fn main() {
    let mut rng = Mcg128Xsl64::from_entropy();
    for i in 0..=16 {
        for j in 0..=16 {
            let (b, w) = Game::new(
                RandomFullSearch::<BitBoard, _>::new(
                    Side::Black,
                    Mcg128Xsl64::from_rng(&mut rng).unwrap(),
                    64 - i,
                ),
                RandomFullSearch::<BitBoard, _>::new(
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
