use nom::IResult;
use nom::combinator::map;
use nom::character::complete::char;

///左括弧のパーサ
pub fn lparen_parser(s: &str) -> IResult<&str, ()> {
    map(char('('), |_| ())(s)
}


///右括弧のパーサ
pub fn rparen_parser(s: &str) -> IResult<&str, ()> {
    map(char(')'), |_| ())(s)
}

#[test]
fn paren_token_parser_test() {
    let result = lparen_parser("(");
    assert_full_match_ok!(result,());

    let result = rparen_parser(")");
    assert_full_match_ok!(result,());
}

