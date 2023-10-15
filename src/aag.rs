use nom::bytes::complete::tag;
use nom::character::complete::alphanumeric1;
use nom::character::complete::newline;
use nom::character::complete::not_line_ending;
use nom::character::complete::one_of;
use nom::character::complete::space1;
use nom::character::complete::u64;
use nom::combinator::all_consuming;
use nom::combinator::flat_map;
use nom::combinator::map;
use nom::multi::count;
use nom::sequence::delimited;
use nom::sequence::terminated;
use nom::sequence::{preceded, tuple};
use nom::Finish;
use nom::IResult;

#[derive(Debug, Clone)]
pub struct Lit {
    pub var: usize,
    pub neg: bool,
}

impl From<u64> for Lit {
    fn from(value: u64) -> Self {
        Self {
            var: (value / 2).try_into().unwrap(),
            neg: (value % 2) == 1,
        }
    }
}

fn literal(input: &[u8]) -> IResult<&[u8], Lit> {
    map(u64, Lit::from)(input)
}

#[derive(Debug)]
pub struct Header {
    /// M = maximum variable index
    pub max_index: u64,
    /// I = number of inputs
    pub inputs: u64,
    /// L = number of latches
    pub latches: u64,
    /// O = number of outputs
    pub outputs: u64,
    /// A = number of AND gates
    pub gates: u64,
}

#[derive(Debug, Clone)]
pub struct A {
    pub lhs: Lit,
    pub rhs0: Lit,
    pub rhs1: Lit,
}

fn header(input: &[u8]) -> IResult<&[u8], Header> {
    delimited(
        tag(b"aag"),
        map(
            tuple((
                space1, u64, space1, u64, space1, u64, space1, u64, space1, u64,
            )),
            |(_, m, _, i, _, l, _, o, _, a)| Header {
                max_index: m,
                inputs: i,
                latches: l,
                outputs: o,
                gates: a,
            },
        ),
        newline,
    )(input)
}

fn parse_gate(input: &[u8]) -> IResult<&[u8], A> {
    terminated(
        map(
            tuple((literal, space1, literal, space1, literal)),
            |(lhs, _, rhs0, _, rhs1)| A { lhs, rhs0, rhs1 },
        ),
        newline,
    )(input)
}

pub fn aag(input: &[u8]) -> Result<(Vec<Lit>, Vec<Lit>, Vec<A>), nom::error::Error<&[u8]>> {
    let result = flat_map(header, |h| {
        tuple((
            count(terminated(literal, newline), h.inputs.try_into().unwrap()),
            count(terminated(literal, newline), h.outputs.try_into().unwrap()),
            count(parse_gate, h.gates.try_into().unwrap()),
        ))
    })(input)
    .finish()?
    .1;

    Ok((result.0, result.1, result.2))
}
