use super::*;

#[test]
fn literal_parser() {
    let parse_joe = match_literal("Hello Joe!");
    assert_eq!(Ok(("", ())), parse_joe("Hello Joe!"));
    assert_eq!(Ok((" Hello Robert!", ())), parse_joe("Hello Joe! Hello Robert!"));
    assert_eq!(
        Err(Error::NotFound(String::from("Hello Mike!"))),
        parse_joe("Hello Mike!")
    );
}
