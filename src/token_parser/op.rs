named!(pub not_parser<()>,do_parse!(
    tag!("not")>>(())
));

#[test]
fn not_token_parser_test() {
    let result = not_parser("not".as_bytes());
    assert_full_match_ok!(result,());

    let result = not_parser("not rtert".as_bytes());
    assert_ok!(result,());
}
