pub mod document;
pub mod terminals;
pub mod statement;
pub mod utils;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct AsciiSequence<'a>(&'a [u8]);

impl<'a> std::fmt::Debug for AsciiSequence<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(self.0))
    }
}
impl<'a> From<&'a [u8]> for AsciiSequence<'a> {
    fn from(s: &'a [u8]) -> Self {
        Self(s)
    }
}
impl<'a> Into<&'a [u8]> for AsciiSequence<'a> {
    fn into(self) -> &'a [u8] {
        self.0
    }
}