use derive_more::*;

#[derive(Debug, Display, From, PartialEq)]
pub enum Error {
    #[display(fmt = "{}", "_0")]
    ParseGrammarViolation(String),
    #[display(fmt = "{}", "_0")]
    InvalidSliceIndex(usize),
}
