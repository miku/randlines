// Emit a subset of lines from a file. Reads line first to figure out line count, then read again
// to emit a line with probability n/lines.

extern crate rand;
use rand::Rng;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::PathBuf;
use structopt::StructOpt;
use tempfile::NamedTempFile;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "randlines",
    about = "emit random subset of lines",
    long_about = "
Emit a random subset of lines from a file. This is a probabilistic program, you
will not get exactly `n` lines.

Typically, you can use shuf(1) which uses reservoir sampling and is very
efficient. However, if we want to extract 10M random lines from a file of 100M
lines, shuf(1) might be killed. However, randlines will not shuffle lines, just
skip over random number of lines.
"
)]
struct Opt {
    #[structopt(short, default_value = "16")]
    n: u64,

    #[structopt(short, long)]
    size_hint: Option<u64>,

    #[structopt(parse(from_os_str))]
    input: Option<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();

    let mut filename: Option<&str> = None;
    let mut file = NamedTempFile::new().expect("failed to create temporary file");
    if opt.input == None {
        io::copy(&mut io::stdin(), &mut file).expect("failed to write to temporary file");
        filename = file.path().to_str();
    }
    let i = match opt.size_hint {
        Some(v) => v,
        None => {
            let reader: Box<dyn BufRead> = match opt.input {
                None => Box::new(BufReader::new(File::open(filename.unwrap()).unwrap())),
                Some(ref f) => Box::new(BufReader::new(File::open(f).unwrap())),
            };
            let mut j = 0;
            for _ in reader.lines() {
                j = j + 1;
            }
            j
        }
    };
    if i == 0 {
        std::process::exit(0);
    }
    let p: f64 = opt.n as f64 / i as f64;

    let reader: Box<dyn BufRead> = match opt.input {
        None => Box::new(BufReader::new(File::open(filename.unwrap()).unwrap())),
        Some(ref f) => Box::new(BufReader::new(File::open(f).unwrap())),
    };
    let stdout = io::stdout();
    let lock = stdout.lock();
    let mut w = io::BufWriter::new(lock);
    let mut rng = rand::thread_rng();
    for line in reader.lines() {
        if rng.gen::<f64>() < p {
            writeln!(w, "{}", line.unwrap()).expect("failed to write line");
        }
    }
}
