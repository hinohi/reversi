use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let mut counts: [[HashMap<_, u64>; 17]; 17] = Default::default();
    let mut reader = BufReader::new(File::open("stats_random.out").unwrap());
    loop {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        if line.is_empty() {
            break;
        }
        let mut words = line.split_ascii_whitespace();
        let i = words.next().unwrap().parse::<usize>().unwrap();
        let j = words.next().unwrap().parse::<usize>().unwrap();
        let b = words.next().unwrap().parse::<u8>().unwrap();
        let w = words.next().unwrap().parse::<u8>().unwrap();
        *counts[i][j].entry((b, w)).or_default() += 1;
    }
    for i in 0..=16 {
        for j in 0..=16 {
            let mut keys = counts[i][j].keys().collect::<Vec<_>>();
            keys.sort();
            for &(b, w) in keys {
                println!("{} {} {} {} {}", i, j, b, w, counts[i][j][&(b, w)]);
            }
        }
    }
}
