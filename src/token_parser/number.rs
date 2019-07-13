use nom::character::complete::digit1;
use std::str;
use std::str::FromStr;

named!(digit_str_parser<&str>,
    map_res!(
        digit1,
        str::from_utf8
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

named_attr!(#[doc="intリテラルのパーサ"],pub int_parser<i32>,
    map_res!(
        do_parse!(
            s: digit_str_parser >>
            alt!(
                map!(eof!(),|_|(())) |
                do_parse!(
                    not!(decimal_parser) >>
                    not!(exponent_parser) >>
                    (())
                )
            ) >>
            (s)
        ),
        FromStr::from_str
    )
);

named_attr!(#[doc="floatリテラルのパーサ"],pub float_parser<f32>,
    map_res!(
        do_parse!(
            s1: digit_str_parser >>
            s2: map!(
                    alt!(
                        map!(eof!(),|_| None) |
                        opt!(decimal_parser)
                    ),
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
