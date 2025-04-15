use crate::utils::take_one;
use crate::AsciiSequence;
use nom::branch::alt;
use nom::bytes::complete::take_while;
use nom::combinator::recognize;
use nom::multi::{many0, many1};
use nom::sequence::preceded;
use nom::{
    bytes::complete::take_while1, error::Error, IResult, Input, OutputMode, PResult, Parser,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Label<'a>(pub AsciiSequence<'a>);
pub fn is_label_token(c: u8) -> bool {
    c.is_ascii_alphanumeric() || c == b'_' || c == b'-' || c == b'.'
}
pub fn label(input: &[u8]) -> IResult<&[u8], Label, Error<&[u8]>> {
    take_while1(is_label_token)
        .map(AsciiSequence::from)
        .map(Label)
        .parse(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Symbol<'a>(pub AsciiSequence<'a>);
pub fn is_symbol_token(c: u8) -> bool {
    if c == b'$' {
        return false;
    }

    c.is_ascii_alphanumeric()
        || b'!' <= c && c <= b'/'
        || b':' <= c && c <= b'@'
        || b'[' <= c && c <= b'`'
        || b'{' <= c && c <= b'~'
}
pub fn symbol(input: &[u8]) -> IResult<&[u8], Symbol, Error<&[u8]>> {
    take_while1(is_symbol_token)
        .map(AsciiSequence::from)
        .map(Symbol)
        .parse(input)
}
impl<'a> Parser<&'a [u8]> for Symbol<'a> {
    type Output = Self;
    type Error = nom::error::Error<&'a [u8]>;

    fn process<OM: OutputMode>(
        &mut self,
        input: &'a [u8],
    ) -> PResult<OM, &'a [u8], Self::Output, Self::Error> {
        symbol.process::<OM>(input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CompressedProof<'a>(pub AsciiSequence<'a>);
pub fn is_compressed_proof_heading_token(c: u8) -> bool {
    b'U' <= c && c <= b'Y'
}
pub fn is_compressed_proof_token(c: u8) -> bool {
    b'A' <= c && c <= b'T'
}
pub fn is_later_occuring_compressed_proof_token(c: u8) -> bool {
    c == b'Z'
}
pub fn compressed_proof(input: &[u8]) -> IResult<&[u8], CompressedProof, Error<&[u8]>> {
    alt((
        take_one(is_later_occuring_compressed_proof_token),
        recognize((
            many0(preceded(ws0, take_one(is_compressed_proof_heading_token))),
            preceded(ws0, take_one(is_compressed_proof_token)),
        )),
    ))
    .map(AsciiSequence::from)
    .map(CompressedProof)
    .parse(input)
}

pub use nom::character::complete::multispace0 as ws0;
pub use nom::character::complete::multispace1 as ws1;
