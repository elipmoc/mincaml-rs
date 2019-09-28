use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;

///boolリテラルのパーサ
pub fn bool_parser(s: &str) -> IResult<&str, bool> {
    map(
        alt((tag("true"), tag("false"))),
        |s| s == "true",
    )(s)
}

#[test]
fn bool_token_parser_test() {
    let result = bool_parser("false");
    assert_full_match_ok!(result,false);
    let result = bool_parser("true");
    assert_full_match_ok!(result,true);
}