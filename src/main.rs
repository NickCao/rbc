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
    let graph = aag(&buf).unwrap().1;

    for thing in &graph.1 {
        println!(
            "output {} is {}",
            *thing,
            &graph.0[*thing as usize].eval(&graph.0, &[true, true])
        );
    }
}
