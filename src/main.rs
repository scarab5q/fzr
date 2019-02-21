extern crate fst;
extern crate fst_levenshtein;
extern crate fst_regex;
extern crate futures;
extern crate tokio;
use tokio::io;
// use tokio::io::BufReader;
use std::io::BufReader;
use tokio::io::{stdin, stdout};

fn main() {
    let reader = BufReader::new(stdin);
    let buffer = Vec::new();

    let fut = io::read_until(reader, b'\n', buffer)
        .and_then(move |(stdin, buffer)| stdout.write_all(&buffer).map_err(|e| panic!(e)))
        .map_err(|e| panic!(e));

    let mut stdin = io::stdin();
    tokio::run(fut);
}
