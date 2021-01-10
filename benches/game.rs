use rand::SeedableRng;
use rand_pcg::Mcg128Xsl64;

use criterion::{criterion_group, criterion_main, Criterion};

use reversi::{
    bit::BitBoard,
    enum1d::Enum1dBoard,
    enum2d::Enum2dBoard,
    search::{RandomFullSearch, RandomSearch},
    Game, Side,
};

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

fn enum1d(c: &mut Criterion) {
    let mut rng = Mcg128Xsl64::new(1);
    c.bench_function("enum1d", |b| {
        b.iter(|| {
            let mut game = Game::new(
                RandomSearch::<Enum1dBoard, _>::new(Mcg128Xsl64::from_rng(&mut rng).unwrap()),
                RandomSearch::<Enum1dBoard, _>::new(Mcg128Xsl64::from_rng(&mut rng).unwrap()),
            );
            let (b, w) = game.play_game();
            assert!(b + w <= 64);
        });
    });
}

fn bit(c: &mut Criterion) {
    let mut rng = Mcg128Xsl64::new(1);
    c.bench_function("bit", |b| {
        b.iter(|| {
            let mut game = Game::new(
                RandomSearch::<BitBoard, _>::new(Mcg128Xsl64::from_rng(&mut rng).unwrap()),
                RandomSearch::<BitBoard, _>::new(Mcg128Xsl64::from_rng(&mut rng).unwrap()),
            );
            let (b, w) = game.play_game();
            assert!(b + w <= 64);
        });
    });
}

fn enum2d_full10(c: &mut Criterion) {
    let mut rng = Mcg128Xsl64::new(1);
    c.bench_function("enum2d_full10", |b| {
        b.iter(|| {
            let mut game = Game::new(
                RandomFullSearch::<Enum2dBoard, _>::new(
                    Side::Black,
                    Mcg128Xsl64::from_rng(&mut rng).unwrap(),
                    64 - 10,
                ),
                RandomFullSearch::<Enum2dBoard, _>::new(
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

fn enum1d_full10(c: &mut Criterion) {
    let mut rng = Mcg128Xsl64::new(1);
    c.bench_function("enum1d_full10", |b| {
        b.iter(|| {
            let mut game = Game::new(
                RandomFullSearch::<Enum1dBoard, _>::new(
                    Side::Black,
                    Mcg128Xsl64::from_rng(&mut rng).unwrap(),
                    64 - 10,
                ),
                RandomFullSearch::<Enum1dBoard, _>::new(
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

fn bit_full10(c: &mut Criterion) {
    let mut rng = Mcg128Xsl64::new(1);
    c.bench_function("bit_full10", |b| {
        b.iter(|| {
            let mut game = Game::new(
                RandomFullSearch::<BitBoard, _>::new(
                    Side::Black,
                    Mcg128Xsl64::from_rng(&mut rng).unwrap(),
                    64 - 10,
                ),
                RandomFullSearch::<BitBoard, _>::new(
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

criterion_group!(
    benches,
    enum2d,
    enum1d,
    bit,
    enum2d_full10,
    enum1d_full10,
    bit_full10,
);
criterion_main!(benches);
