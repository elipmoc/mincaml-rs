use nom::{IResult, InputLength};
use nom::character::complete::{one_of, char};
use nom::sequence::preceded;
use nom::combinator::{peek, map};
use nom::multi::many1;
use nom::branch::alt;
use nom::Err;
use nom::error::ErrorKind;

define_ignore_str_parser!(#[doc=""],if_parser,"if");
define_ignore_str_parser!(#[doc=""],then_parser,"then");
define_ignore_str_parser!(#[doc=""],else_parser,"else");
define_ignore_str_parser!(#[doc=""],let_parser,"let");
define_ignore_str_parser!(#[doc=""],in_parser,"in");
define_ignore_str_parser!(#[doc=""],rec_parser,"rec");
define_ignore_str_parser!(#[doc=""],comma_parser,",");
define_ignore_str_parser!(#[doc=""],ident_ignore_parser,"_");
define_ignore_str_parser!(#[doc=""],array_create_parser,"Array.create");
define_ignore_str_parser!(#[doc=""],dot_parser,".");
define_ignore_str_parser!(#[doc=""],less_minus_parser,"<-");
define_ignore_str_parser!(#[doc=""],semicolon_parser,";");

pub fn eof_parser(s: &str) -> IResult<&str, &str> {
    if s.input_len() == 0 {
        Ok((s, s))
    } else {
        Err(Err::Error(error_position!(s,ErrorKind::Eof)))
    }
}

fn lower_parser(s: &str) -> IResult<&str, char> {
    one_of("abcdefghijklmnopqrstuvwxyz")(s)
}

fn upper_parser(s: &str) -> IResult<&str, char> {
    one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ")(s)
}

fn number_parser(s: &str) -> IResult<&str, char> {
    one_of("0123456789")(s)
}

///小文字からはじまる識別子のパーサ
pub fn ident_lower_parser(s: &str) -> IResult<&str, String> {
    preceded(
        peek(lower_parser),
        map(
            many1(alt((
                lower_parser,
                upper_parser,
                number_parser,
                char('_')
            ))),
            |v| v.iter().collect(),
        ),
    )(s)
}

#[test]
fn ident_lower_parser_test() {
    let result = ident_lower_parser("aaabb77cc");
    assert_full_match_ok!(result,"aaabb77cc");

    let result = ident_lower_parser("aaabb77cc GGG");
    assert_ok!(result,"aaabb77cc");

    let result = ident_lower_parser("77rgads");
    assert_err!(result);

    let result = ident_lower_parser("R");
    assert_err!(result);

    let result = ident_lower_parser("hogeFuga_777");
    assert_ok!(result,"hogeFuga_777");
}