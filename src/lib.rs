#![warn(clippy::all, clippy::nursery, clippy::pedantic, rust_2018_idioms)]
#![forbid(bare_trait_objects)]
#![allow(clippy::match_bool)]
// To use the `unsafe` keyword, change to `#![allow(unsafe_code)]` (do not remove); aids auditing.
#![forbid(unsafe_code)]
// Safety-critical application lints
#![deny(
    clippy::pedantic,
    clippy::float_cmp_const,
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::option_unwrap_used,
    clippy::result_unwrap_used
)]

// Uncomment before ship to reconcile use of possibly redundant crates, debug remnants, missing license files and more
//#![warn(clippy::cargo, clippy::restriction, missing_docs, warnings)]
//#![deny(warnings)]

#[cfg(test)]
mod unit_tests;

mod consts;
mod error;
pub use error::Error;
use std::result::Result as StdResult;

// suppress false negative (failed to detect use in `match_literal()`'s return type)
#[allow(dead_code)]
type Result<T> = StdResult<T, Error>;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Element {
    name: String,
    attributes: Vec<(String, String)>,
    children: Vec<Element>,
}

pub fn match_literal(expected: &'static str) -> impl Fn(&str) -> Result<(&str, ())> {
    move |input| match input.get(..expected.len()) {
        Some(next_input) if next_input == expected => Ok((
            input
                .get(expected.len()..)
                .ok_or_else(|| Error::InvalidSliceIndex(expected.len()))?,
            (),
        )),
        _ => Err(Error::NotFound(String::from(input))),
    }
}

pub fn identifier(input: &str) -> Result<(&str, String)> {
    let mut matched = String::new();
    let mut chars = input.chars();

    match chars.next() {
        Some(next_input) if next_input.is_alphabetic() => matched.push(next_input),
        _ => return Err(Error::NotFound(String::from(input))),
    }

    while let Some(next_input) = chars.next() {
        match next_input.is_alphanumeric() || next_input == '-' {
            true => matched.push(next_input),
            false => break,
        }
    }

    Ok((
        input
            .get(matched.len()..)
            .ok_or_else(|| Error::InvalidSliceIndex(matched.len()))?,
        matched,
    ))
}

pub fn pair<P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Fn(&str) -> Result<(&str, (R1, R2))>
where
    P1: Fn(&str) -> Result<(&str, R1)>,
    P2: Fn(&str) -> Result<(&str, R2)>,
{
    move |input| match parser1(input) {
        Ok((next_input, result1)) => match parser2(next_input) {
            Ok((final_input, result2)) => Ok((final_input, (result1, result2))),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}
