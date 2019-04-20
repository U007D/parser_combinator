use parser_combinator::*;

#[test]
fn xml_parser() {
    let doc = r###"
        <top label="Top">
            <semi-bottom label="Bottom"/>
            <middle>
                <bottom label="Another bottom"/>
            </middle>
        </top>"###;
    let parsed_doc = Element {
        name: "top".to_string(),
        attributes: vec![("label".to_string(), "Top".to_string())],
        children: vec![
            Element {
                name: "semi-bottom".to_string(),
                attributes: vec![("label".to_string(), "Bottom".to_string())],
                children: vec![],
            },
            Element {
                name: "middle".to_string(),
                attributes: vec![],
                children: vec![Element {
                    name: "bottom".to_string(),
                    attributes: vec![("label".to_string(), "Another bottom".to_string())],
                    children: vec![],
                }],
            },
        ],
    };
    assert_eq!(Ok(("", parsed_doc)), element().parse(doc));
}

#[test]
fn mismatched_closing_tag() {
    let doc = r###"
        <top>
            <bottom/>
        </middle>"###;
    assert_eq!(
        Err(Error::ParseGrammarViolation(String::from("</middle>"))),
        element().parse(doc)
    );
}
