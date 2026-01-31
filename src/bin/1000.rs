use std::io::{self, BufRead, BufWriter};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let mut input = String::new();
    stdin.lock().read_line(&mut input).unwrap();

    let mut iter = input.split_whitespace();
    let a: i32 = iter.next().unwrap().parse().unwrap();
}
