named!(pub bool_parser<bool>,do_parse!(
    bool : alt!(
        map!(tag!("true"),|_| true ) |
        map!(tag!("false"),|_| false )
    ) >>
    (bool)
));

#[test]
fn bool_token_parser_test() {
    let result = bool_parser("false".as_bytes());
    assert_full_match_ok!(result,false);
    let result = bool_parser("true".as_bytes());
    assert_full_match_ok!(result,true);
}