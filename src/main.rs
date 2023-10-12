use clap::Parser;
use rbc::{aag, Gate, Input};
use std::{collections::HashMap, io::Read};

/// RBC: System for Combinational Logic Synthesis
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {}

fn main() {
    let args = Args::parse();
    let mut buf = vec![];
    std::io::stdin().read_to_end(&mut buf).unwrap();
    let mut graph = aag(&buf).unwrap().1;

    for x in [0, 1] {
        for y in [0, 1] {
            graph.simulate(&[x, y]);
        }
    }
}
