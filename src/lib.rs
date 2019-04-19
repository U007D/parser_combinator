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
    clippy::result_unwrap_use
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

type Result<T> = StdResult<T, Error>;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Element {
    name: String,
    attributes: Vec<(String, String)>,
    children: Vec<Element>,
}

pub fn match_literal(expected: &'static str) -> impl Fn(&str) -> Result<(&str, ())> {
    move |input| match input.get(..expected.len()) {
        Some(s) if s == expected => Ok((
            input
                .get(expected.len()..)
                .ok_or_else(|| Error::InvalidSliceIndex('a'.len_utf8()))?,
            (),
        )),
        _ => Err(Error::NotFound(String::from(input))),
    }
}
