use rand::SeedableRng;
use rand_pcg::Mcg128Xsl64;

use criterion::{criterion_group, criterion_main, Criterion};

use reversi::{
    search::{RandomFullSearch, RandomSearch},
    Game, Side,
};

fn bit(c: &mut Criterion) {
    let mut rng = Mcg128Xsl64::new(1);
    c.bench_function("bit", |b| {
        b.iter(|| {
            let mut game = Game::new(
                RandomSearch::new(Mcg128Xsl64::from_rng(&mut rng).unwrap()),
                RandomSearch::new(Mcg128Xsl64::from_rng(&mut rng).unwrap()),
            );
            let (b, w) = game.play_game();
            assert!(b + w <= 64);
        });
    });
}

fn bit_full10(c: &mut Criterion) {
    let mut rng = Mcg128Xsl64::new(1);
    c.bench_function("bit_full10", |b| {
        b.iter(|| {
            let mut game = Game::new(
                RandomFullSearch::new(
                    Side::Black,
                    Mcg128Xsl64::from_rng(&mut rng).unwrap(),
                    64 - 10,
                ),
                RandomFullSearch::new(
                    Side::White,
                    Mcg128Xsl64::from_rng(&mut rng).unwrap(),
                    64 - 10,
                ),
            );
            let (b, w) = game.play_game();
            assert!(b + w <= 64);
        });
    });
}

criterion_group!(benches, bit, bit_full10);
criterion_main!(benches);
