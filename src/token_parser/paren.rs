named_attr!(#[doc="左括弧のパーサ"],pub lparen_parser<()>,
do_parse!(
    tag!("(") >> (())
));

named_attr!(#[doc="右括弧のパーサ"],pub rparen_parser<()>,
do_parse!(
    tag!(")") >> (())
));

#[test]
fn paren_token_parser_test() {
    let result = lparen_parser("(".as_bytes());
    assert_full_match_ok!(result,());

    let result = rparen_parser(")".as_bytes());
    assert_full_match_ok!(result,());
}

