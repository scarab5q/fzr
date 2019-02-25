extern crate fst;
extern crate fst_levenshtein;
extern crate fst_regex;
extern crate futures;
// extern crate tokio;
// use tokio::io;
// use tokio::io::BufReader;
// use std::io::BufReader;
// use tokio::io::{Stdin, Stdout};
use std::io::{self, BufRead};

fn read_stdin() {
    let stdin = io::stdin();
    let mut keys = Vec::new();
    for line in stdin.lock().lines() {
        let key: String = line.unwrap();
        keys.push(key);
    }
    println!("{:?}", keys);
}

fn main() {
    read_stdin();
}
