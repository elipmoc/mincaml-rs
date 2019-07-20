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

named!(lower_parser<char>, one_of!("abcdefghijklmnopqrstuvwxyz"));
named!(upper_parser<char>, one_of!("ABCDEFGHIJKLMNOPQRSTUVWXYZ"));
named!(number_parser<char>, one_of!("0123456789"));
named_attr!(#[doc="小文字からはじまる識別子のパーサ"],pub ident_lower_parser<String>,
    preceded!(
        peek!(alt!(lower_parser)),
        map!(
            many1!(complete!(alt!(
                lower_parser |
                upper_parser |
                number_parser |
                char!('_')
            ))),
            |v|v.iter().collect()
        )
    )
);

#[test]
fn ident_lower_parser_test() {
    let result = ident_lower_parser("aaabb77cc".as_bytes());
    assert_full_match_ok!(result,"aaabb77cc");

    let result = ident_lower_parser("aaabb77cc GGG".as_bytes());
    assert_ok!(result,"aaabb77cc");

    let result = ident_lower_parser("77rgads".as_bytes());
    assert_err!(result);

    let result = ident_lower_parser("R".as_bytes());
    assert_err!(result);

    let result = ident_lower_parser("hogeFuga_777".as_bytes());
    assert_ok!(result,"hogeFuga_777");
}