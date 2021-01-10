use rand::SeedableRng;
use rand_pcg::Mcg128Xsl64;

use reversi::{bit::BitBoard, enum2d::Enum2dBoard, search::RandomSearch, Game};

#[test]
fn smoke_enum2d_bit() {
    let mut rng = Mcg128Xsl64::new(1);
    for _ in 0..100 {
        let mut game = Game::new(
            RandomSearch::<Enum2dBoard, _>::new(Mcg128Xsl64::from_rng(&mut rng).unwrap()),
            RandomSearch::<BitBoard, _>::new(Mcg128Xsl64::from_rng(&mut rng).unwrap()),
        );
        let (b, w) = game.play_game();
        assert!(b + w <= 64);
        let mut game = Game::new(
            RandomSearch::<BitBoard, _>::new(Mcg128Xsl64::from_rng(&mut rng).unwrap()),
            RandomSearch::<Enum2dBoard, _>::new(Mcg128Xsl64::from_rng(&mut rng).unwrap()),
        );
        let (b, w) = game.play_game();
        assert!(b + w <= 64);
    }
}
