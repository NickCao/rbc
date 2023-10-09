use clap::Parser;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_till;
use nom::character::complete::alpha1;
use nom::character::complete::alphanumeric1;
use nom::character::complete::newline;
use nom::character::complete::one_of;
use nom::character::complete::space1;
use nom::character::complete::u64;
use nom::character::is_newline;
use nom::combinator::map;
use nom::multi::count;
use nom::multi::many0;
use nom::sequence::delimited;
use nom::sequence::terminated;
use nom::sequence::{preceded, tuple};
use nom::IResult;
use std::io::Read;

/// RBC: System for Combinational Logic Synthesis
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {}

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

#[derive(Debug)]
struct Symbol {
    kind: char,
    variable: u64,
    identifier: String,
}

#[derive(Debug)]
struct Input {
    i: Literal,
}

#[derive(Debug)]
struct Output {
    o: Literal,
}

#[derive(Debug)]
struct Latch {
    i: Literal,
    o: Literal,
}

#[derive(Debug)]
struct Gate {
    a: Literal,
    b: Literal,
    o: Literal,
}

#[derive(Debug)]
struct Literal {
    variable: u64,
    negate: bool,
}

impl From<u64> for Literal {
    fn from(value: u64) -> Self {
        Self {
            variable: value / 2,
            negate: value % 2 == 1,
        }
    }
}

fn header(input: &[u8]) -> IResult<&[u8], Header> {
    delimited(
        tag(b"aag"),
        map(
            tuple((
                space1, u64, space1, u64, space1, u64, space1, u64, space1, u64,
            )),
            |(_, m, _, i, _, l, _, o, _, a)| Header { m, i, l, o, a },
        ),
        newline,
    )(input)
}

fn parse_input(input: &[u8]) -> IResult<&[u8], Input> {
    terminated(map(u64, |i| Input { i: i.into() }), newline)(input)
}

fn parse_output(input: &[u8]) -> IResult<&[u8], Output> {
    terminated(map(u64, |o| Output { o: o.into() }), newline)(input)
}

fn parse_latch(input: &[u8]) -> IResult<&[u8], Latch> {
    terminated(
        map(tuple((space1, u64, space1, u64)), |(_, i, _, o)| Latch {
            i: i.into(),
            o: o.into(),
        }),
        newline,
    )(input)
}

fn parse_gate(input: &[u8]) -> IResult<&[u8], Gate> {
    terminated(
        map(tuple((u64, space1, u64, space1, u64)), |(o, _, a, _, b)| {
            Gate {
                a: a.into(),
                b: b.into(),
                o: o.into(),
            }
        }),
        newline,
    )(input)
}

fn parse_symbol(input: &[u8]) -> IResult<&[u8], Symbol> {
    terminated(
        map(tuple((one_of("ilo"), u64)), |(kind, variable)| Symbol {
            kind,
            variable,
            identifier: "test".to_string(),
        }),
        newline,
    )(input)
    /*
    terminated(
        map(
            tuple((alphanumeric1, space1, alphanumeric1)),
            |(kind, _, identifier)| Symbol {
                kind,
                variable,
                identifier: identifier.to_string(),
            },
        ),
        newline,
    )(input)
    */
}

fn aig(input: &[u8]) -> IResult<&[u8], Header> {
    let h = header(input)?;
    let d = tuple((
        count(parse_input, h.1.i.try_into().unwrap()),
        count(parse_latch, h.1.l.try_into().unwrap()),
        count(parse_output, h.1.o.try_into().unwrap()),
        count(parse_gate, h.1.a.try_into().unwrap()),
        many0(parse_symbol),
    ))(h.0)?;
    dbg!(d);
    Ok(h)
}

fn main() {
    let args = Args::parse();
    let mut buf = vec![];
    std::io::stdin().read_to_end(&mut buf).unwrap();
    dbg!(aig(&buf).unwrap());
}
