use nom::error::{Error, ErrorKind, ParseError};
use nom::{IResult, Input};

// pub fn map_second<T, U>(mut f: impl FnMut(T) -> U) -> impl FnMut((&[u8], T)) -> (&[u8], U) {
//     move |(a, b)| (a, f(b))
// }

pub fn take_one(
    predicate: impl Fn(u8) -> bool,
) -> impl FnMut(&[u8]) -> IResult<&[u8], &[u8], Error<&[u8]>> {
    move |input| {
        let mut iter = input.iter_elements();
        match iter.next() {
            Some(c) if predicate(c) => {
                let (remaining, matched) = input.take_split(1);
                Ok((remaining, matched))
            }
            _ => Err(nom::Err::Error(Error::from_error_kind(
                input,
                ErrorKind::Satisfy,
            ))),
        }
    }
}

// pub fn compose<T, U, V>(
//     mut f: impl FnMut(T) -> U,
//     mut g: impl FnMut(U) -> V,
// ) -> impl FnMut(T) -> V {
//     move |x| g(f(x))
// }

// #[macro_export]
// macro_rules! mapsc {
//     ($head:expr) => {
//         map_second($head)
//     };
//     ($head:expr, $($tail:expr),*) => {
//         map_second(crate::compose!($head, $($tail),+))
//     };
// }

// #[macro_export]
// macro_rules! compose {
//     ($last:expr) => { $last };
//     ($head:expr, $($tail:expr),+) => {
//         move |x| crate::compose!($($tail),+)($head(x))
//     };
// }
