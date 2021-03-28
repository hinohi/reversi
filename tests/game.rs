use rand::SeedableRng;
use rand_pcg::Mcg128Xsl64;

use reversi::{search::RandomSearch, Game};

#[test]
fn smoke_enum2d_bit() {
    let mut rng = Mcg128Xsl64::new(1);
    for _ in 0..100 {
        let mut game = Game::new(
            RandomSearch::new(Mcg128Xsl64::from_rng(&mut rng).unwrap()),
            RandomSearch::new(Mcg128Xsl64::from_rng(&mut rng).unwrap()),
        );
        let (b, w) = game.play_game();
        assert!(b + w <= 64);
        let mut game = Game::new(
            RandomSearch::new(Mcg128Xsl64::from_rng(&mut rng).unwrap()),
            RandomSearch::new(Mcg128Xsl64::from_rng(&mut rng).unwrap()),
        );
        let (b, w) = game.play_game();
        assert!(b + w <= 64);
    }
}
