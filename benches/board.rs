use rand::SeedableRng;
use rand_pcg::Mcg128Xsl64;

use criterion::{criterion_group, criterion_main, Criterion};

use reversi::{enum2d::Enum2dBoard, search::RandomSearch, Game};

fn enum2d(c: &mut Criterion) {
    let mut rng = Mcg128Xsl64::new(1);
    c.bench_function("enum2d", |b| {
        b.iter(|| {
            let mut game = Game::new(
                RandomSearch::<Enum2dBoard, _>::new(Mcg128Xsl64::from_rng(&mut rng).unwrap()),
                RandomSearch::<Enum2dBoard, _>::new(Mcg128Xsl64::from_rng(&mut rng).unwrap()),
            );
            let (b, w) = game.play_game();
            assert!(b + w <= 64);
        });
    });
}

criterion_group!(benches, enum2d);
criterion_main!(benches);
