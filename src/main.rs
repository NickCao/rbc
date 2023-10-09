use clap::Parser;
use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::character::complete::space1;
use nom::character::complete::u64;
use nom::combinator::map;
use nom::sequence::delimited;
use nom::sequence::{preceded, tuple};
use nom::IResult;
use std::io::Read;

/// RBC: System for Combinational Logic Synthesis
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {}

#[allow(non_snake_case)]
#[derive(Debug)]
struct Header {
    /// M = maximum variable index
    m: u64,
    /// I = number of inputs
    i: u64,
    /// L = number of latches
    l: u64,
    /// O = number of outputs
    o: u64,
    /// A = number of AND gates
    a: u64,
}

fn header(input: &[u8]) -> IResult<&[u8], Header> {
    delimited(
        tag(b"aig"),
        map(
            tuple((
                space1, u64, space1, u64, space1, u64, space1, u64, space1, u64,
            )),
            |(_, m, _, i, _, l, _, o, _, a)| Header { m, i, l, o, a },
        ),
        newline,
    )(input)
}

fn aig(input: &[u8]) -> IResult<&[u8], Header> {
    let h = header(input)?;
    Ok(h)
}

fn main() {
    let args = Args::parse();
    let mut buf = vec![];
    std::io::stdin().read_to_end(&mut buf).unwrap();
    dbg!(aig(&buf).unwrap());
}
