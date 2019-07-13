named!(lparen_parser<()>,do_parse!(
    tag!("(") >> (())
));

named!(rparen_parser<()>,do_parse!(
    tag!(")") >> (())
));

#[test]
fn paren_token_parser_test() {
    let result = lparen_parser("(".as_bytes());
    assert_full_match_ok!(result,());

    let result = rparen_parser(")".as_bytes());
    assert_full_match_ok!(result,());
}

