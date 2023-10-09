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
use nom::combinator::opt;
use nom::multi::count;
use nom::multi::many0;
use nom::multi::many1;
use nom::sequence::delimited;
use nom::sequence::terminated;
use nom::sequence::{preceded, tuple};
use nom::IResult;

#[derive(Debug)]
pub enum Node {
    Input,
    Gate,
}

#[derive(Debug)]
pub struct AIG(Vec<Option<Node>>);

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

#[derive(Debug)]
pub struct Symbol {
    pub kind: char,
    pub variable: u64,
    pub identifier: String,
}

#[derive(Debug)]
pub struct Input(pub Literal);

#[derive(Debug)]
pub struct Output(pub Literal);

#[derive(Debug)]
pub struct Latch(pub Literal, pub Literal);

#[derive(Debug)]
pub struct Gate(pub Literal, pub Literal, pub Literal);

#[derive(Debug)]
pub struct Literal {
    pub variable: u64,
    pub negate: bool,
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

fn parse_input(input: &[u8]) -> IResult<&[u8], Input> {
    terminated(map(u64, |i| Input(i.into())), newline)(input)
}

fn parse_output(input: &[u8]) -> IResult<&[u8], Output> {
    terminated(map(u64, |o| Output(o.into())), newline)(input)
}

fn parse_latch(input: &[u8]) -> IResult<&[u8], Latch> {
    terminated(
        map(tuple((space1, u64, space1, u64)), |(_, i, _, o)| {
            Latch(o.into(), i.into())
        }),
        newline,
    )(input)
}

fn parse_gate(input: &[u8]) -> IResult<&[u8], Gate> {
    terminated(
        map(tuple((u64, space1, u64, space1, u64)), |(o, _, a, _, b)| {
            Gate(o.into(), a.into(), b.into())
        }),
        newline,
    )(input)
}

fn parse_symbol(input: &[u8]) -> IResult<&[u8], Symbol> {
    terminated(
        map(
            tuple((one_of("ilo"), u64::<&[u8], _>, space1, alphanumeric1)),
            |(kind, variable, _, identifier)| Symbol {
                kind,
                variable,
                identifier: String::from_utf8(identifier.to_vec()).unwrap(),
            },
        ),
        newline,
    )(input)
}

fn parse_comment(input: &[u8]) -> IResult<&[u8], String> {
    terminated(
        map(not_line_ending, |s: &[u8]| {
            String::from_utf8(s.to_vec()).unwrap()
        }),
        newline,
    )(input)
}

pub fn aag(
    input: &[u8],
) -> IResult<
    &[u8],
    (
        Vec<Input>,
        Vec<Latch>,
        Vec<Output>,
        Vec<Gate>,
        Vec<Symbol>,
        Option<Vec<String>>,
    ),
> {
    all_consuming(flat_map(header, |h| {
        tuple((
            count(parse_input, h.inputs.try_into().unwrap()),
            count(parse_latch, h.latches.try_into().unwrap()),
            count(parse_output, h.outputs.try_into().unwrap()),
            count(parse_gate, h.gates.try_into().unwrap()),
            many0(parse_symbol),
            opt(preceded(tag(b"c\n"), many1(parse_comment))),
        ))
    }))(input)
}
