use crate::{parser::Parser, tokenizer::Tokenizer};

#[test]
fn nil_true_false() {
    let input = "nil";
    let expected = "nil";

    assert_eq!(parse(input), expected.to_string());

    let input = "true";
    let expected = "true";

    assert_eq!(parse(input), expected.to_string());

    let input = "false";
    let expected = "false";

    assert_eq!(parse(input), expected.to_string());
}

#[test]
fn binaries() {
    let input = "1 + 2";
    let expected = "(+ 1.0 2.0)";

    assert_eq!(parse(input), expected.to_string());

    let input = "1 - 2";
    let expected = "(- 1.0 2.0)";

    assert_eq!(parse(input), expected.to_string());

    let input = "1 / 2";
    let expected = "(/ 1.0 2.0)";

    assert_eq!(parse(input), expected.to_string());

    let input = "1 * 2";
    let expected = "(* 1.0 2.0)";

    assert_eq!(parse(input), expected.to_string());
}

#[test]
fn unaries() {
    let input = "!false";
    let expected = "(! false)";

    assert_eq!(parse(input), expected.to_string());

    let input = "4 + -1";
    let expected = "(+ 4.0 (- 1.0 ))";

    assert_eq!(parse(input), expected.to_string());

    let input = "4 * -1";
    let expected = "(* 4.0 (- 1.0 ))";

    assert_eq!(parse(input), expected.to_string());
}

#[test]
fn grouping() {
    let input = "(1 + 3) + 5";
    let expected = "(+ (group (+ 1.0 3.0)) 5.0)";

    assert_eq!(parse(input), expected.to_string());

    let input = "(1 + 3) + \"hi\"";
    let expected = "(+ (group (+ 1.0 3.0)) \"hi\")";

    assert_eq!(parse(input), expected.to_string());

    let input = "(1 + 3) + (3 * 8) / 2";
    let expected = "(+ (group (+ 1.0 3.0)) (/ (group (* 3.0 8.0)) 2.0))";

    assert_eq!(parse(input), expected.to_string());

    let input = "((true))";
    let expected = "(group (group true))";

    assert_eq!(parse(input), expected.to_string());
}

#[test]
fn order_of_opreations() {
    let input = "1 + 3 * 2";
    let expected = "(+ 1.0 (* 3.0 2.0))";

    assert_eq!(parse(input), expected.to_string());

    let input = "1 + 3 * 2 + 4";
    let expected = "(+ (+ 1.0 (* 3.0 2.0)) 4.0)";

    assert_eq!(parse(input), expected.to_string());

    let input = "1 + 3 * 2 / 4";
    let expected = "(+ 1.0 (/ (* 3.0 2.0) 4.0))";

    assert_eq!(parse(input), expected.to_string());

    let input = "1 + 3 * 5 / 2 + 4";
    let expected = "(+ (+ 1.0 (/ (* 3.0 5.0) 2.0)) 4.0)";

    assert_eq!(parse(input), expected.to_string());
}

#[test]
fn complex() {
    let input = "((true) + 1.5 - 34 * 21 / (2.53 - -1))";
    let expected = "(group (- (+ (group true) 1.5) (/ (* 34.0 21.0) (group (- 2.53 (- 1.0 ))))))";

    assert_eq!(parse(input), expected.to_string());
}

fn parse(input: &str) -> String {
    format!(
        "{}",
        Parser::parse_tokens(&Tokenizer::tokenize(input.to_string()).unwrap().get_tokens())
    )
}
