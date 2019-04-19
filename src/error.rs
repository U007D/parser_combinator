use derive_more::*;

#[derive(Debug, Display, From, PartialEq)]
pub enum Error {
    #[display(fmt = "{}", "_0")]
    NotFound(String),
    #[display(fmt = "{}", "_0")]
    InvalidSliceIndex(usize),
}
