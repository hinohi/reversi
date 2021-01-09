use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

use reversi::{naive::NaiveBoard, Board, Side};

fn replay_scenario<P, B>(scenario: P, mut board: B)
where
    P: AsRef<Path>,
    B: Board,
{
    let mut scenario =
        BufReader::new(File::open(scenario).expect("scenario file expected")).lines();

    fn compare<B: Board>(scenario: &mut impl Iterator<Item = io::Result<String>>, board: &B) {
        let mut expect = String::with_capacity(9 * 8);
        for _ in 0..8 {
            expect.push_str(&scenario.next().unwrap().unwrap());
            expect.push('\n');
        }
        let actual = format!("{}", board);
        assert_eq!(expect, actual);
    };

    while let Some(result) = scenario.next() {
        let s = result.unwrap();
        let mut cmd = s.split_ascii_whitespace();
        match cmd.next().unwrap() {
            "given" => compare(&mut scenario, &board),
            "put" => {
                let side = match cmd.next().expect("put [BW] col row") {
                    "B" => Side::Black,
                    "W" => Side::White,
                    _ => unreachable!(),
                };
                let col = cmd.next().unwrap().parse().expect("put BW [col] row");
                let row = cmd.next().unwrap().parse().expect("put BW col [row]");
                println!("pub {:?} {} {}", side, col, row);
                board.put(col, row, side);
                compare(&mut scenario, &board);
            }
            c => panic!(format!("Unsupported command: {}", c)),
        }
    }
}

#[test]
fn replay001_naive() {
    replay_scenario("tests/board_cases/001.txt", NaiveBoard::default());
}
