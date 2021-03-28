use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

use reversi::{BitBoard, Side};

fn replay_scenario<P>(scenario: P, mut board: BitBoard)
where
    P: AsRef<Path>,
{
    let mut scenario =
        BufReader::new(File::open(scenario).expect("scenario file expected")).lines();

    trait Scenario: Iterator<Item = io::Result<String>> {}
    impl<T: Iterator<Item = io::Result<String>>> Scenario for T {}
    trait Args<'a>: Iterator<Item = &'a str> {}
    impl<'a, T: Iterator<Item = &'a str>> Args<'a> for T {}

    fn compare<S: Scenario>(board: &BitBoard, scenario: &mut S) {
        let mut expect = String::with_capacity(9 * 8);
        for _ in 0..8 {
            expect.push_str(&scenario.next().unwrap().unwrap());
            expect.push('\n');
        }
        let mut actual = Vec::new();
        board.format(&mut actual).unwrap();
        assert_eq!(expect, String::from_utf8(actual).unwrap());
    }

    fn candidates<'a, A: Args<'a>, S: Scenario>(
        board: &BitBoard,
        args: &'a mut A,
        scenario: &mut S,
    ) {
        let mut expect = Vec::new();
        for r in 0..8 {
            let row = scenario.next().unwrap().unwrap();
            for (c, ch) in row.chars().enumerate() {
                match ch {
                    '*' => {
                        expect.push(BitBoard::position(c, r));
                    }
                    '●' | '○' | '_' | '\n' => (),
                    ch => panic!("Unexpected char: {}", ch),
                };
            }
        }
        let side = match args.next().expect("candidates [BW]") {
            "B" => Side::Black,
            "W" => Side::White,
            _ => panic!("candidates [BW]"),
        };
        expect.sort();
        let mut actual = board.candidates(side).collect::<Vec<_>>();
        actual.sort();
        assert_eq!(expect, actual);
    }

    fn put<'a, A: Args<'a>>(board: &mut BitBoard, args: &'a mut A) {
        let side = match args.next().expect("put [BW] col row") {
            "B" => Side::Black,
            "W" => Side::White,
            _ => panic!("put [BW] col row"),
        };
        let col: usize = args.next().unwrap().parse().expect("put BW [col] row");
        let row: usize = args.next().unwrap().parse().expect("put BW col [row]");
        println!("put {:?} {} {}", side, col, row);
        board.put(side, BitBoard::position(col, row));
    }

    fn count<'a, A: Args<'a>>(board: &BitBoard, args: &'a mut A) {
        let b = args.next().unwrap().parse().expect("count [B] W");
        let w = args.next().unwrap().parse().expect("count B [W]");
        assert_eq!(board.count(), (b, w));
    }

    while let Some(result) = scenario.next() {
        let s = result.unwrap();
        let mut cmd = s.split_ascii_whitespace();
        match cmd.next() {
            Some("compare") => compare(&board, &mut scenario),
            Some("put") => put(&mut board, &mut cmd),
            Some("count") => count(&board, &mut cmd),
            Some("candidates") => candidates(&board, &mut cmd, &mut scenario),
            Some(c) => panic!("Unsupported command: {}", c),
            None => (),
        }
    }
}

#[test]
fn replay001_bit() {
    replay_scenario("tests/board_cases/001.txt", BitBoard::default());
}

#[test]
fn replay002_bit() {
    replay_scenario("tests/board_cases/002.txt", BitBoard::default());
}
