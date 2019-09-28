use nom::character::complete::{digit1, char};
use nom::combinator::{map, value, map_res};
use nom::sequence::{preceded, tuple};
use nom::branch::alt;
use nom::combinator::{opt, not};
use std::str;
use std::str::FromStr;
use nom::IResult;
use crate::token_parser::eof_parser;

fn digit_str_parser(s: &str) -> IResult<&str, &str> {
    digit1(s)
}

fn decimal_parser(s: &str) -> IResult<&str, String> {
    map(
        preceded(char('.'), digit_str_parser),
        |s| ".".to_string() + s,
    )(s)
}

fn exponent_parser(s: &str) -> IResult<&str, String> {
    let e_parser = alt((char('e'), char('E')));
    let sign_parser =
        map(
            opt(
                alt((char('+'), char('-')))
            ),
            |sign_o| sign_o.unwrap_or('+'),
        );
    map(
        tuple((e_parser, sign_parser, digit_str_parser)),
        |(e, sign, d)| e.to_string() + &sign.to_string() + d,
    )(s)
}

fn unwrap_or_empty_string(so: Option<String>) -> String {
    so.unwrap_or("".to_string())
}

///intリテラルのパーサ
pub fn int_parser(s: &str) -> IResult<&str, i32> {
    map_res(
        tuple((
            digit_str_parser,
            alt((
                value((), eof_parser),
                value((), tuple((not(decimal_parser), not(exponent_parser))))
            ))
        )),
        |(s, _)| FromStr::from_str(s),
    )(s)
}

///floatリテラルのパーサ
pub fn float_parser(s: &str) -> IResult<&str, f32> {
    let s1 = digit_str_parser;
    let s2 = map(
        alt((
            value(None, eof_parser),
            opt(decimal_parser)
        )),
        unwrap_or_empty_string,
    );
    let s3 = map(
        alt((
            value(None, eof_parser),
            opt(exponent_parser)
        )),
        unwrap_or_empty_string,
    );
    map_res(
        map(
            tuple((s1, s2, s3)),
            |(s1, s2, s3)| s1.to_string() + &s2 + &s3,
        ),
        |s: String| FromStr::from_str(s.as_str()),
    )(s)
}

#[test]
fn number_token_parser_test() {
    let result = int_parser("123");
    assert_full_match_ok!(result,123);

    let result = int_parser("455sadfas47");
    assert_ok!(result,455);

    let result = float_parser("1.48");
    assert_full_match_ok!(result,1.48f32);

    let result = float_parser("1e4");
    assert_full_match_ok!(result,1e4f32);

    let result = float_parser("1e-4");
    assert_full_match_ok!(result,1e-4f32);

    let result = float_parser("1E+5");
    assert_full_match_ok!(result,1e+5f32);

    let result = float_parser("1.5E-2");
    assert_full_match_ok!(result,1.5E-2f32);
}
