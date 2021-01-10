use rand_pcg::Mcg128Xsl64;

use reversi::{enum2d::Enum2dBoard, search::RandomSearch, Game};

#[test]
fn smoke_random_random() {
    let mut game = Game::new(
        RandomSearch::<Enum2dBoard, _>::new(Mcg128Xsl64::new(1)),
        RandomSearch::<Enum2dBoard, _>::new(Mcg128Xsl64::new(3)),
    );
    let (b, w) = game.play_game();
    println!("{} {}", b, w);
    assert!(b + w <= 64);
}
