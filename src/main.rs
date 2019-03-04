mod fsaps;

extern crate fst;
extern crate fst_levenshtein;
extern crate fst_regex;
extern crate futures;

use std::io::{self, BufRead};

use fst_regex::Regex;
use std::env;
use std::str;

fn read_stdin() -> Vec<String> {
    let stdin: Vec<String> = io::stdin().lock().lines().map(|x| x.unwrap()).collect();
    stdin
}

fn main() {
    let stdin = read_stdin();
    let results: Vec<String>;

    for line in stdin {}

    // let string: Vec<u8> = fsaps::read_line().as_bytes().to_vec();
    // let pattern: Vec<u8> = fsaps::read_line().as_bytes().to_vec();

    // fsaps::kmp(&string, &pattern);
}
