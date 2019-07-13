/// not演算子のパーサ
define_ignore_str_parser!(not_parser,"not");

/// -演算子のパーサ
define_ignore_str_parser!(minus_parser,"-");

/// +演算子のパーサ
define_ignore_str_parser!(plus_parser,"+");

// その他よくわからんパーサ
define_ignore_str_parser!(plus_dot_parser,"+.");
define_ignore_str_parser!(ast_dot_parser,"*.");
define_ignore_str_parser!(slash_dot_parser,"/.");
define_ignore_str_parser!(equal_parser,"=");
define_ignore_str_parser!(less_greater_parser,"<>");
define_ignore_str_parser!(less_equal_parser,"<=");
define_ignore_str_parser!(greater_equal_parser,">=");
define_ignore_str_parser!(less_parser,"<");
define_ignore_str_parser!(greater_parser,">");

#[test]
fn not_token_parser_test() {
    let result = not_parser("not".as_bytes());
    assert_full_match_ok!(result,());

    let result = not_parser("not rtert".as_bytes());
    assert_ok!(result,());
}
