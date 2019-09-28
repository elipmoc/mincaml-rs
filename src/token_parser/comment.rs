use nom::IResult;
use nom::bytes::complete::{tag, take};
use nom::combinator::{not, value};
use nom::sequence::{tuple, preceded};
use nom::multi::many0;
use nom::branch::alt;

fn comment_begin_parser(s: &str) -> IResult<&str, ()> {
    value((), tag("(*"))(s)
}

fn comment_end_parser(s: &str) -> IResult<&str, ()> {
    value((), tag("*)"))(s)
}

///コメント文のパーサ
pub fn comment_parser(s: &str) -> IResult<&str, ()> {
    value(
        (),
        tuple((
            comment_begin_parser,
            many0(
                alt((
                    comment_parser,
                    value((), preceded(not(comment_end_parser), take(1usize)))
                ))
            ),
            comment_end_parser
        )),
    )(s)
}

#[test]
fn comment_token_parser_test() {
    let result = comment_begin_parser("(*");
    assert_full_match_ok!(result,());

    let result = comment_end_parser("*)");
    assert_full_match_ok!(result,());

    let result = comment_parser("(**)");
    assert_full_match_ok!(result,());

    let result = comment_parser("(*aaa iii ううう*)");
    assert_full_match_ok!(result,());

    let result = comment_parser("(*aaa iii う)うう**)");
    assert_full_match_ok!(result,());

    let result = comment_parser("(*aaa iii(* aa *) う)うう**)");
    assert_full_match_ok!(result,());
}