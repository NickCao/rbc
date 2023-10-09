use clap::Parser;
use rbc::aag;
use std::io::Read;

/// RBC: System for Combinational Logic Synthesis
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {}

fn main() {
    let args = Args::parse();
    let mut buf = vec![];
    std::io::stdin().read_to_end(&mut buf).unwrap();
    dbg!(aag(&buf).unwrap());
}
