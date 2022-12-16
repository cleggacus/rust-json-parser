use json_parser::{parser::{Parser, node::Value}};

#[test]
fn nasa() {
    let mut parser = Parser::new("tests/nasa.json");

    let val = match parser.parse_value() {
        None => panic!("Got none from parser!"),
        Some(val) => val,
    };

    let barnaul = &val[78]["name"];

    match barnaul {
        Value::String(val) => assert_eq!(val, "Barnaul"),
        _ => panic!("Incorrect type received!")
    };
}