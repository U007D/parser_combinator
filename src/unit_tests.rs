use super::*;

#[test]
fn literal_parser() {
    let parse_joe = match_literal("Hello Joe!");
    assert_eq!(Ok(("", ())), parse_joe.parse("Hello Joe!"));
    assert_eq!(Ok((" Hello Robert!", ())), parse_joe.parse("Hello Joe! Hello Robert!"));
    assert_eq!(
        Err(Error::NotFound(String::from("Hello Mike!"))),
        parse_joe.parse("Hello Mike!")
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
    let tag1 = String::from(tag1_open) + tag1_attr + tag1_close;
    let tag2_attr = "oops!";
    let tag3_attr = "!oops";
    let tag3_incomplete = String::from(tag1_open) + tag3_attr;
    let tag_opener = pair(match_literal("<"), identifier);

    assert_eq!(Ok((tag1_close, ((), String::from(tag1_attr)))), tag_opener.parse(&tag1));
    assert_eq!(
        Err(Error::NotFound(String::from(tag2_attr))),
        tag_opener.parse(tag2_attr)
    );
    assert_eq!(
        Err(Error::NotFound(String::from(tag3_attr))),
        tag_opener.parse(&tag3_incomplete)
    );
}

#[test]
fn right_combinator() {
    let tag1_open = "<";
    let tag1_attr = "my-first-element";
    let tag1_close = "/>";
    let tag1 = String::from(tag1_open) + tag1_attr + tag1_close;
    let tag2_attr = "oops!";
    let tag3_attr = "!oops";
    let tag3_incomplete = String::from(tag1_open) + tag3_attr;
    let right = right(match_literal(tag1_open), identifier);
    assert_eq!(Ok((tag1_close, String::from(tag1_attr))), right.parse(&tag1));
    assert_eq!(Err(Error::NotFound(String::from(tag2_attr))), right.parse(tag2_attr));
    assert_eq!(
        Err(Error::NotFound(String::from(tag3_attr))),
        right.parse(&tag3_incomplete)
    );
}
