use crate::util::*;
use nom::character::complete::digit1;
use std::str;
use std::str::FromStr;

named!(comment_begin_parser<()>,do_parse!(
    tag!("(*")>>(())
));

named!(comment_end_parser<()>,do_parse!(
    tag!("*)")>>(())
));

named!(comment_parser<()>,do_parse!(
    comment_begin_parser >>
    many0!(
        alt!(
           comment_parser |
           map!(preceded!(not!(comment_end_parser),take!(1)),|_|(()))
        )
       ) >>
    comment_end_parser >>
    (())
));

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
    ) >>
    (bool)
));

named!(not_parser<()>,do_parse!(
    tag!("not")>>(())
));

named!(digit_str_parser<&str>,
    map_res!(
        digit1,
        str::from_utf8
    )
);

named!(int_parser<i32>,
    map_res!(
        digit_str_parser,
        FromStr::from_str
    )
);

named!(decimal_parser<String>,
    map!(
        preceded!( char!('.'), digit_str_parser),
        |s|".".to_string()+s
    )
);

named!(exponent_parser<String>,
    do_parse!(
        e: alt!( char!('e') | char!('E') ) >>
        sign: map!(opt!( alt!( char!('+') | char!('-') ) ),|sign_o| sign_o.unwrap_or('+') ) >>
        d: digit_str_parser >>
        (e.to_string()+&sign.to_string()+d)
    )
);

fn unwrap_or_empty_string(so:Option<String>)->String{
    so.unwrap_or("".to_string())
}

named!(float_parser<f32>,
    map_res!(
        do_parse!(
            s1: digit_str_parser >>
            s2: map!(
                    opt!(decimal_parser),
                    unwrap_or_empty_string
                ) >>
            s3: map!(
                    alt!(
                        map!(eof!(),|_| None) |
                        opt!(exponent_parser)
                    ),
                    unwrap_or_empty_string
                ) >>
            (s1.to_string()+&s2+&s3)
        ),
        |s:String| FromStr::from_str(s.as_str())
    )
);

#[test]
fn bool_token_parser_test() {
    let result = bool_parser("false".as_bytes());
    assert_full_match_ok!(result,false);
    let result = bool_parser("true".as_bytes());
    assert_full_match_ok!(result,true);
}

#[test]
fn not_token_parser_test() {
    let result = not_parser("not".as_bytes());
    assert_full_match_ok!(result,());

    let result = not_parser("not rtert".as_bytes());
    assert_ok!(result,());
}

#[test]
fn paren_token_parser_test() {
    let result = lparen_parser("(".as_bytes());
    assert_full_match_ok!(result,());

    let result = rparen_parser(")".as_bytes());
    assert_full_match_ok!(result,());
}

#[test]
fn comment_token_parser_test() {
    let result = comment_begin_parser("(*".as_bytes());
    assert_full_match_ok!(result,());

    let result = comment_end_parser("*)".as_bytes());
    assert_full_match_ok!(result,());

    let result = comment_parser("(**)".as_bytes());
    assert_full_match_ok!(result,());

    let result = comment_parser("(*aaa iii ううう*)".as_bytes());
    assert_full_match_ok!(result,());

    let result = comment_parser("(*aaa iii う)うう**)".as_bytes());
    assert_full_match_ok!(result,());

    let result = comment_parser("(*aaa iii(* aa *) う)うう**)".as_bytes());
    assert_full_match_ok!(result,());
}

#[test]
fn number_token_parser_test() {
    let result = int_parser("123".as_bytes());
    assert_full_match_ok!(result,123);

    let result = int_parser("455sadfas47".as_bytes());
    assert_ok!(result,455);

    let result = float_parser("1.48".as_bytes());
    assert_full_match_ok!(result,1.48f32);

    let result = float_parser("1e4".as_bytes());
    assert_full_match_ok!(result,1e4f32);

    let result = float_parser("1e-4".as_bytes());
    assert_full_match_ok!(result,1e-4f32);

    let result = float_parser("1E+5".as_bytes());
    assert_full_match_ok!(result,1e+5f32);

    let result = float_parser("1.5E-2".as_bytes());
    assert_full_match_ok!(result,1.5E-2f32);
}
