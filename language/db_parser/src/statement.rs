use crate::document::{major_part_heading_comment, section, sub_section, sub_sub_section};
use crate::terminals::{compressed_proof, label, symbol, ws0, ws1, CompressedProof, Label, Symbol};
use crate::AsciiSequence;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::combinator::not;
use nom::multi::{many0, many1, separated_list1, separated_list0};
use nom::sequence::{preceded, terminated};
use nom::{error::Error, sequence::delimited, IResult, Parser};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Statement<'a> {
    ConstantDeclaration(ConstantDeclaration<'a>),
    VariableDeclaration(VariableDeclaration<'a>),
    DisjoinVariableDeclaration(DisjoinVariableDeclaration<'a>),
    FloatingHypothesis(FloatingHypothesis<'a>),
    EssentialHypothesis(EssentialHypothesis<'a>),
    AxiomaticAssertion(AxiomaticAssertion<'a>),
    ProvableAssertion(ProvableAssertion<'a>),
    Block(Vec<Box<Statement<'a>>>),
    Comment(Comment<'a>),
    // IncludeFile(IncludeFile<'a>),
    // Unit,
}
pub fn statement(input: &[u8]) -> IResult<&[u8], Statement, Error<&[u8]>> {
    alt((
        constant_declaration.map(Statement::ConstantDeclaration),
        variable_declaration.map(Statement::VariableDeclaration),
        disjoin_variable_declaration.map(Statement::DisjoinVariableDeclaration),
        floating_hypothesis.map(Statement::FloatingHypothesis),
        essential_hypothesis.map(Statement::EssentialHypothesis),
        axiomatic_assertion.map(Statement::AxiomaticAssertion),
        provable_assertion.map(Statement::ProvableAssertion),
        block.map(Statement::Block),
        comment.map(Statement::Comment),
    ))
    .parse(input)
}
// pub fn statement(input: &[u8]) -> IResult<&[u8], Statement, Error<&[u8]>> {
//     alt((
//         constant_declaration.map(|_| ()),
//         variable_declaration.map(|_| ()),
//         disjoin_variable_declaration.map(|_| ()),
//         floating_hypothesis.map(|_| ()),
//         essential_hypothesis.map(|_| ()),
//         axiomatic_assertion.map(|_| ()),
//         provable_assertion.map(|_| ()),
//         block.map(|_| ()),
//         comment.map(|_| ()),
//     ))
//     .map(|_| Statement::Unit)
//     .parse(input)
// }

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConstantDeclaration<'a> {
    pub symbol_list: Vec<Symbol<'a>>,
}
impl<'a> ConstantDeclaration<'a> {
    pub fn new(symbol_list: Vec<Symbol<'a>>) -> Self {
        Self { symbol_list }
    }
}
pub fn constant_declaration(input: &[u8]) -> IResult<&[u8], ConstantDeclaration, Error<&[u8]>> {
    delimited(
        (tag("$c"), ws1),
        separated_list1(ws1, symbol),
        (ws1, tag("$.")),
    )
    .map(ConstantDeclaration::new)
    .parse(input)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VariableDeclaration<'a> {
    symbol_list: Vec<Symbol<'a>>,
}
impl<'a> VariableDeclaration<'a> {
    pub fn new(symbol_list: Vec<Symbol<'a>>) -> Self {
        Self { symbol_list }
    }
}
pub fn variable_declaration(input: &[u8]) -> IResult<&[u8], VariableDeclaration, Error<&[u8]>> {
    delimited(
        (tag("$v"), ws1),
        separated_list1(ws1, symbol),
        (ws1, tag("$.")),
    )
    .map(VariableDeclaration::new)
    .parse(input)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DisjoinVariableDeclaration<'a> {
    symbol_list: Vec<Symbol<'a>>,
}
impl<'a> DisjoinVariableDeclaration<'a> {
    pub fn new(symbol_list: Vec<Symbol<'a>>) -> Self {
        Self { symbol_list }
    }
}
pub fn disjoin_variable_declaration(
    input: &[u8],
) -> IResult<&[u8], DisjoinVariableDeclaration, Error<&[u8]>> {
    delimited(
        (tag("$d"), ws1),
        separated_list1(ws1, symbol),
        (ws1, tag("$.")),
    )
    .map(DisjoinVariableDeclaration::new)
    .parse(input)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FloatingHypothesis<'a> {
    label: Label<'a>,
    symbol_list: Vec<Symbol<'a>>,
}
impl<'a> FloatingHypothesis<'a> {
    pub fn new((label, symbol_list): (Label<'a>, Vec<Symbol<'a>>)) -> Self {
        Self { label, symbol_list }
    }
}
pub fn floating_hypothesis(input: &[u8]) -> IResult<&[u8], FloatingHypothesis, Error<&[u8]>> {
    (
        terminated(label, ws1),
        delimited(
            (tag("$f"), ws1),
            separated_list1(ws1, symbol),
            (ws1, tag("$.")),
        ),
    )
        .map(FloatingHypothesis::new)
        .parse(input)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EssentialHypothesis<'a> {
    label: Label<'a>,
    symbol_list: Vec<Symbol<'a>>,
}
impl<'a> EssentialHypothesis<'a> {
    pub fn new((label, symbol_list): (Label<'a>, Vec<Symbol<'a>>)) -> Self {
        Self { label, symbol_list }
    }
}
pub fn essential_hypothesis(input: &[u8]) -> IResult<&[u8], EssentialHypothesis, Error<&[u8]>> {
    (
        terminated(label, ws1),
        delimited(
            (tag("$e"), ws1),
            separated_list1(ws1, symbol),
            (ws1, tag("$.")),
        ),
    )
        .map(EssentialHypothesis::new)
        .parse(input)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AxiomaticAssertion<'a> {
    pub label: Label<'a>,
    pub symbol_list: Vec<Symbol<'a>>,
}
impl<'a> AxiomaticAssertion<'a> {
    pub fn new((label, symbol_list): (Label<'a>, Vec<Symbol<'a>>)) -> Self {
        Self { label, symbol_list }
    }
}
pub fn axiomatic_assertion(input: &[u8]) -> IResult<&[u8], AxiomaticAssertion, Error<&[u8]>> {
    (
        terminated(label, ws1),
        delimited(
            (tag("$a"), ws1),
            separated_list1(ws1, symbol),
            (ws1, tag("$.")),
        ),
    )
        .map(AxiomaticAssertion::new)
        .parse(input)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProvableAssertion<'a> {
    label: Label<'a>,
    symbol_list: Vec<Symbol<'a>>,
    proof: Proof<'a>,
}
impl<'a> ProvableAssertion<'a> {
    pub fn new((label, symbol_list, proof): (Label<'a>, Vec<Symbol<'a>>, Proof<'a>)) -> Self {
        Self {
            label,
            symbol_list,
            proof,
        }
    }
}
pub fn provable_assertion(input: &[u8]) -> IResult<&[u8], ProvableAssertion, Error<&[u8]>> {
    (
        terminated(label, ws1),
        delimited(
            (tag("$p"), ws1),
            separated_list1(ws1, symbol),
            (ws1, tag("$="), ws1),
        ),
        terminated(proof, (ws0, tag("$."))),
    )
        .map(ProvableAssertion::new)
        .parse(input)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Proof<'a> {
    labels: Vec<Label<'a>>,
    compressed_proofs: Vec<CompressedProof<'a>>,
}
impl<'a> Proof<'a> {
    pub fn new((labels, compressed_proofs): (Vec<Label<'a>>, Vec<CompressedProof<'a>>)) -> Self {
        Self {
            labels,
            compressed_proofs,
        }
    }
}
pub fn proof(input: &[u8]) -> IResult<&[u8], Proof, Error<&[u8]>> {
    (
        delimited(
            tag("("),
            many0(preceded(ws1, label)),
            (ws0, tag(")")),
        ),
        many1(preceded(ws0, compressed_proof)),
    )
        .map(Proof::new)
        .parse(input)
}

pub fn block(input: &[u8]) -> IResult<&[u8], Vec<Box<Statement>>, Error<&[u8]>> {
    delimited(tag("${"), many0(preceded(ws0, statement)), (ws0, tag("$}"))) 
        .map(|statements| statements.into_iter().map(Box::new).collect())
        .parse(input)
}
// pub fn block(input: &[u8]) -> IResult<&[u8], (), Error<&[u8]>> {
//     delimited(tag("${"), many0(preceded(ws0, statement)), (ws0, tag("$}"))) 
//         .map(|_| ())
//         .parse(input)
// }

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Comment<'a> {
    comment: AsciiSequence<'a>,
}
impl<'a> Comment<'a> {
    pub fn new(comment: AsciiSequence<'a>) -> Self {
        Self { comment }
    }
}
pub fn comment(input: &[u8]) -> IResult<&[u8], Comment, Error<&[u8]>> {
    not(major_part_heading_comment)
        .and(not(section))
        .and(not(sub_section))
        .and(not(sub_sub_section))
        .and(delimited(
            (tag("$("), ws0),
            take_until("$)"),
            (ws0, tag("$)")),
        ))
        .map(|(_, c)| c)
        .map(AsciiSequence::from)
        .map(Comment::new)
        .parse(input)
}
