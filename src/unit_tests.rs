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

#[test]
fn one_or_more_combinator() {
    let parser = one_or_more(match_literal("ha"));
    assert_eq!(Ok(("", vec![(), (), ()])), parser.parse("hahaha"));
    assert_eq!(Err(Error::NotFound(String::from("ahah"))), parser.parse("ahah"));
    assert_eq!(Err(Error::NotFound(String::new())), parser.parse(""));
}

#[test]
fn zero_or_more_combinator() {
    let parser = zero_or_more(match_literal("ha"));
    assert_eq!(Ok(("", vec![(), (), ()])), parser.parse("hahaha"));
    assert_eq!(Ok(("ahah", vec![])), parser.parse("ahah"));
    assert_eq!(Ok(("", vec![])), parser.parse(""));
}

#[test]
fn any_combinator() {
    let checker = any_char;
    assert_eq!(Ok(("", 'x')), checker("x"));
    assert_eq!(Err(Error::NotFound(String::new())), checker(""));
}

#[test]
fn predicate_combinator() {
    let parser = pred(any_char, |c| *c == 'o');
    assert_eq!(Ok(("mg", 'o')), parser.parse("omg"));
    assert_eq!(Err(Error::NotFound(String::from("lol"))), parser.parse("lol"));
}

#[test]
fn quoted_string_parser() {
    assert_eq!(
        Ok(("", String::from("Hello, Joe!"))),
        quoted_string().parse("\"Hello, Joe!\"")
    );
}

#[test]
fn attribute_parser() {
    assert_eq!(
        Ok((
            "",
            vec![
                (String::from("one"), String::from("1")),
                (String::from("two"), String::from("2")),
            ]
        )),
        attributes().parse(" one=\"1\" two=\"2\"")
    );
}

//#[test]
//fn single_element_parser() {
//    assert_eq!(
//        Ok((
//            "",
//            Element {
//                name: String::from("div"),
//                attributes: vec![(String::from("class"), String::from("float"))],
//                children: vec![]
//            }
//        )),
//        single_element().parse("<div class=\"float\"/>")
//    );
//}
