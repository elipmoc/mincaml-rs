use crate::util::*;

named!(lparen_parser<()>,do_parse!(
    tag!("(") >> (())
));

named!(rparen_parser<()>,do_parse!(
    tag!(")") >> (())
));

named!(bool_parser<bool>,do_parse!(
    bool : alt!(
        map!(tag!("true"),|_| true ) |
        map!(tag!("false"),|_| false )
    )
    >>(bool)
));

named!(not_parser<()>,do_parse!(
    tag!("not")>>(())
));

#[test]
fn bool_token_parser_test() {
    let result = bool_parser("false".as_bytes());
    assert_ok!(result,false);

    let result = bool_parser("true".as_bytes());
    assert_ok!(result,true);
}

#[test]
fn not_token_parser_test() {
    let result = not_parser("not".as_bytes());
    assert_ok!(result,());

    let result = not_parser("not rtert".as_bytes());
    assert_ok!(result,());
}

#[test]
fn paren_token_parser_test() {
    let result = lparen_parser("(".as_bytes());
    assert_ok!(result,());

    let result = rparen_parser(")".as_bytes());
    assert_ok!(result,());
}
