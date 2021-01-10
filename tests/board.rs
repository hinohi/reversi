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

    trait Scenario: Iterator<Item = io::Result<String>> {}
    impl<T: Iterator<Item = io::Result<String>>> Scenario for T {}
    trait Args<'a>: Iterator<Item = &'a str> {}
    impl<'a, T: Iterator<Item = &'a str>> Args<'a> for T {}

    fn compare<B: Board, S: Scenario>(board: &B, scenario: &mut S) {
        let mut expect = String::with_capacity(9 * 8);
        for _ in 0..8 {
            expect.push_str(&scenario.next().unwrap().unwrap());
            expect.push('\n');
        }
        let actual = format!("{}", board);
        assert_eq!(expect, actual);
    }

    fn candidates<'a, B: Board, A: Args<'a>, S: Scenario>(
        board: &B,
        args: &'a mut A,
        scenario: &mut S,
    ) {
        let mut expect = Vec::new();
        for r in 0..8 {
            let row = scenario.next().unwrap().unwrap();
            for (c, ch) in row.chars().enumerate() {
                match ch {
                    '*' => {
                        expect.push((c, r));
                    }
                    '●' | '○' | '_' | '\n' => (),
                    ch => panic!(format!("Unexpected char: {}", ch)),
                };
            }
        }
        let side = match args.next().expect("candidates [BW]") {
            "B" => Side::Black,
            "W" => Side::White,
            _ => panic!("candidates [BW]"),
        };
        expect.sort();
        let mut actual = board.list_candidates(side);
        actual.sort();
        assert_eq!(expect, actual);
    }

    fn put<'a, B: Board, A: Args<'a>>(board: &mut B, args: &'a mut A) {
        let side = match args.next().expect("put [BW] col row") {
            "B" => Side::Black,
            "W" => Side::White,
            _ => panic!("put [BW] col row"),
        };
        let col = args.next().unwrap().parse().expect("put BW [col] row");
        let row = args.next().unwrap().parse().expect("put BW col [row]");
        println!("put {:?} {} {}", side, col, row);
        board.put(col, row, side);
    }

    fn count<'a, B: Board, A: Args<'a>>(board: &B, args: &'a mut A) {
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
            Some(c) => panic!(format!("Unsupported command: {}", c)),
            None => (),
        }
    }
}

#[test]
fn replay001_naive() {
    replay_scenario("tests/board_cases/001.txt", NaiveBoard::default());
}
