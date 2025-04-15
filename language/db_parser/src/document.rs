use crate::statement::{comment, statement, Comment, Statement};
use crate::terminals::ws0;
use crate::AsciiSequence;
use nom::bytes::complete::{tag, take_until, take_while};
use nom::character::complete::char;
use nom::multi::{many0, many_m_n, separated_list1};
use nom::sequence::delimited;
use nom::Parser;
use nom::{multi::many1, sequence::preceded};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Document<'a> {
    pub heading_comments: Vec<Comment<'a>>,
    pub major_parts: Vec<MajorPart<'a>>,
}
impl<'a> Document<'a> {
    pub fn new((heading_comments, major_parts): (Vec<Comment<'a>>, Vec<MajorPart<'a>>)) -> Self {
        Self {
            heading_comments,
            major_parts,
        }
    }
}
pub fn document(input: &[u8]) -> nom::IResult<&[u8], Document, nom::error::Error<&[u8]>> {
    (
        many0(preceded(ws0, comment)),
        many0(preceded(ws0, major_part)),
    )
        .map(Document::new)
        .parse(input)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MajorPart<'a> {
    pub title: AsciiSequence<'a>,
    pub heading_comments: AsciiSequence<'a>,
    pub statements: Vec<Statement<'a>>,
    pub sections: Vec<Section<'a>>,
}
impl<'a> MajorPart<'a> {
    pub fn new(
        ((title, heading_comments), statements, sections): (
            (AsciiSequence<'a>, AsciiSequence<'a>),
            Vec<Statement<'a>>,
            Vec<Section<'a>>,
        ),
    ) -> Self {
        Self {
            title,
            heading_comments,
            statements,
            sections,
        }
    }
}
pub fn major_part_heading_comment<'a>(
    input: &'a [u8],
) -> nom::IResult<&'a [u8], (AsciiSequence<'a>, AsciiSequence<'a>), nom::error::Error<&'a [u8]>> {
    heading_comment('#', '#').parse(input)
}
pub fn major_part(input: &[u8]) -> nom::IResult<&[u8], MajorPart, nom::error::Error<&[u8]>> {
    (
        major_part_heading_comment,
        many0(preceded(ws0, statement)),
        many0(preceded(ws0, section)),
    )
        .map(MajorPart::new)
        .parse(input)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Section<'a> {
    pub title: AsciiSequence<'a>,
    pub heading_comments: AsciiSequence<'a>,
    pub statements: Vec<Statement<'a>>,
    pub subsections: Vec<SubSection<'a>>,
}
impl<'a> Section<'a> {
    pub fn new(
        ((title, heading_comments), statements, subsections): (
            (AsciiSequence<'a>, AsciiSequence<'a>),
            Vec<Statement<'a>>,
            Vec<SubSection<'a>>,
        ),
    ) -> Self {
        Self {
            title,
            heading_comments,
            statements,
            subsections,
        }
    }
}
pub fn section_heading_comment<'a>(
    input: &'a [u8],
) -> nom::IResult<&'a [u8], (AsciiSequence<'a>, AsciiSequence<'a>), nom::error::Error<&'a [u8]>> {
    heading_comment('*', '#').parse(input)
}
pub fn section(input: &[u8]) -> nom::IResult<&[u8], Section, nom::error::Error<&[u8]>> {
    (
        section_heading_comment,
        many0(preceded(ws0, statement)),
        many0(preceded(ws0, sub_section)),
    )
        .map(Section::new)
        .parse(input)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SubSection<'a> {
    pub title: AsciiSequence<'a>,
    pub heading_comments: AsciiSequence<'a>,
    pub statements: Vec<Statement<'a>>,
    pub subsubsections: Vec<SubSubSection<'a>>,
}
impl<'a> SubSection<'a> {
    pub fn new(
        ((title, heading_comments), statements, subsubsections): (
            (AsciiSequence<'a>, AsciiSequence<'a>),
            Vec<Statement<'a>>,
            Vec<SubSubSection<'a>>,
        ),
    ) -> Self {
        Self {
            title,
            heading_comments,
            statements,
            subsubsections,
        }
    }
}
pub fn sub_section_heading_comment<'a>(
    input: &'a [u8],
) -> nom::IResult<&'a [u8], (AsciiSequence<'a>, AsciiSequence<'a>), nom::error::Error<&'a [u8]>> {
    heading_comment('-', '=').parse(input)
}
pub fn sub_section(input: &[u8]) -> nom::IResult<&[u8], SubSection, nom::error::Error<&[u8]>> {
    (
        sub_section_heading_comment,
        many0(preceded(ws0, statement)),
        many0(preceded(ws0, sub_sub_section)),
    )
        .map(SubSection::new)
        .parse(input)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SubSubSection<'a> {
    pub title: AsciiSequence<'a>,
    pub heading_comments: AsciiSequence<'a>,
    pub statements: Vec<Statement<'a>>,
}
impl<'a> SubSubSection<'a> {
    pub fn new(
        ((title, heading_comments), statements): (
            (AsciiSequence<'a>, AsciiSequence<'a>),
            Vec<Statement<'a>>,
        ),
    ) -> Self {
        Self {
            title,
            heading_comments,
            statements,
        }
    }
}
pub fn sub_sub_section_heading_comment<'a>(
    input: &'a [u8],
) -> nom::IResult<&'a [u8], (AsciiSequence<'a>, AsciiSequence<'a>), nom::error::Error<&'a [u8]>> {
    heading_comment('.', '-').parse(input)
}
pub fn sub_sub_section(
    input: &[u8],
) -> nom::IResult<&[u8], SubSubSection, nom::error::Error<&[u8]>> {
    (
        sub_sub_section_heading_comment,
        many0(preceded(ws0, statement)),
    )
        .map(SubSubSection::new)
        .parse(input)
}

pub fn heading_comment<'a>(
    inner: char,
    outter: char,
) -> impl Parser<
    &'a [u8],
    Output = (AsciiSequence<'a>, AsciiSequence<'a>),
    Error = nom::error::Error<&'a [u8]>,
> + 'a {
    delimited(
        tag("$("),
        (
            preceded(ws0, delimited_separated_list1(inner, outter)).map(AsciiSequence::from),
            take_until("$)").map(AsciiSequence::from),
        ),
        tag("$)"),
    )
}

pub fn delimited_separated_list1<'a>(
    inner: char,
    outter: char,
) -> impl Parser<&'a [u8], Output = AsciiSequence<'a>, Error = nom::error::Error<&'a [u8]>> + 'a {
    delimited(
        separated_list1(char(inner), char(outter)),
        delimited(ws0, take_while(|c: u8| c != b'\n'), ws0),
        separated_list1(char(inner), char(outter)),
    )
    .map(AsciiSequence::from)
}
