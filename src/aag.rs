use nom::{
    bytes::complete::tag,
    character::complete::{newline, space1, u64},
    combinator::{flat_map, map},
    multi::count,
    sequence::{delimited, preceded, terminated, tuple},
    Finish, IResult,
};

struct Header {
    i: u64,
    l: u64,
    o: u64,
    a: u64,
}

fn header(input: &[u8]) -> IResult<&[u8], Header> {
    preceded(
        tag(b"aag"),
        map(
            tuple((
                space1, u64, space1, u64, space1, u64, space1, u64, space1, u64,
            )),
            |(_, _, _, i, _, l, _, o, _, a)| Header { i, l, o, a },
        ),
    )(input)
}

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

#[derive(Debug, Clone)]
pub struct And {
    pub lhs: Lit,
    pub rhs0: Lit,
    pub rhs1: Lit,
}

fn and(input: &[u8]) -> IResult<&[u8], And> {
    map(
        tuple((literal, delimited(space1, literal, space1), literal)),
        |(lhs, rhs0, rhs1)| And { lhs, rhs0, rhs1 },
    )(input)
}

pub fn parse(input: &[u8]) -> Result<(Vec<Lit>, Vec<Lit>, Vec<And>), nom::error::Error<&[u8]>> {
    Ok(flat_map(terminated(header, newline), |h| {
        assert_eq!(h.l, 0);
        tuple((
            count(terminated(literal, newline), h.i as usize),
            count(terminated(literal, newline), h.o as usize),
            count(terminated(and, newline), h.a as usize),
        ))
    })(input)
    .finish()?
    .1)
}
