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

#[test]
fn identifier_parser() {
    let in_out_str = "I-am-an-identifier";
    assert_eq!(Ok(("", String::from(in_out_str))), identifier(in_out_str));

    let in_string = String::from("not");
    let out_str = " entirely an identifier";
    let full_string = in_string.clone() + out_str;
    assert_eq!(Ok((out_str, in_string)), identifier(&full_string));

    let full_str = "!not at all an identifier";
    assert_eq!(Err(Error::NotFound(String::from(full_str))), identifier(full_str));
}

#[test]
fn pair_combinator() {
    let tag1_open = "<";
    let tag1_attr = "my-first-element";
    let tag1_close = "/>";
    let tag2 = "oops!";
    let tag3 = "!oops";
    let tag_opener = pair(match_literal("<"), identifier);

    assert_eq!(
        Ok((tag1_close, ((), String::from(tag1_attr)))),
        tag_opener(&(String::from(tag1_open) + tag1_attr + tag1_close))
    );
    assert_eq!(Err(Error::NotFound(String::from(tag2))), tag_opener(tag2));
    assert_eq!(
        Err(Error::NotFound(String::from(tag3))),
        tag_opener(&(String::from(tag1_open) + tag3))
    );
}
