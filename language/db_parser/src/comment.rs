#[derive(Debug)]
pub struct AdditionalInformationComment<'a>(AsciiSequence<'a>);

#[derive(Debug)]
pub struct TypesettingComment<'a>(AsciiSequence<'a>);

#[derive(Debug)]
pub struct TitleComment<'a>(AsciiSequence<'a>);

#[derive(Debug)]
pub struct DescriptionComment<'a>(AsciiSequence<'a>);

#[derive(Debug)]
pub enum AuthorComment<'a> {
    ContributedBy(AsciiSequence<'a>),
    ProofShortenedBy(AsciiSequence<'a>),
}
impl<'a> AuthorComment<'a> {
    pub fn db_parse(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let (input, authors) = delimited(
            tag("("),
            alt((
                map(
                    Self::start_with_until_paren_close("Contributed by"),
                    |x| Self::ContributedBy(x),
                ),
                map(
                    Self::start_with_until_paren_close("Proof shortened by"),
                    |x| Self::ProofShortenedBy(x),
                ),
            )),
            tag(")"),
        )
        .parse(input)?;
        Ok((input, authors))
    }

    pub fn start_with_until_paren_close(start: &'a str) -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], AsciiSequence<'a>> + 'a {
        move |input| Ok((recognize((tag(start), take_while(|c: u8| c != b')'))).map(AsciiSequence::from).parse(input)?))
    }
}