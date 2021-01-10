use rand_pcg::Mcg128Xsl64;

use reversi::{naive::NaiveBoard, search::RandomSearch, Game};

#[test]
fn smoke_random_random() {
    let mut game = Game::new(
        RandomSearch::<NaiveBoard, _>::new(Mcg128Xsl64::new(1)),
        RandomSearch::<NaiveBoard, _>::new(Mcg128Xsl64::new(3)),
    );
    let (b, w) = game.play_game();
    println!("{} {}", b, w);
    assert!(b + w <= 64);
}
