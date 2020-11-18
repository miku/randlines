// Emit a subset of lines from a file. Reads line first to figure out line count, then read again
// to emit a line with probability n/lines.

extern crate rand;
use rand::Rng;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let usage = "usage: randlines -n NUM FILE

Emit a random subset of lines from a file. This is a probabilistic
program, you will not get exactly `n` lines, but about `n` lines.

🈹 Typically, you can use shuf(1) which uses reservoir sampling and
is very efficient. However, if we want to extract 10M random lines
from a file of 100M lines, shuf(1) might be killed.

EXAMPLE

    randlines -n 1000 my.file";

    let args: Vec<String> = env::args().collect();
    if args.len() < 4 || args[1] != "-n" {
        println!("{}", usage);
        std::process::exit(1);
    }
    let mut n: u64 = 0;
    let mut i: u64 = 0;
    if args[1] == "-n" {
        n = args[2].parse::<u64>().unwrap();
    }
    let f = File::open(&args[3]).expect("unable to open file");
    let f = BufReader::new(f);
    for _ in f.lines() {
        i = i + 1;
    }
    if i == 0 {
        std::process::exit(0);
    }
    let p: f64 = n as f64 / i as f64;
    let f = File::open(&args[3]).expect("unable to open file");
    let f = BufReader::new(f);
    let mut rng = rand::thread_rng();
    for line in f.lines() {
        let line = line.expect("unable to read line");
        if rng.gen::<f64>() < p {
            println!("{}\n", line);
        }
    }
}
