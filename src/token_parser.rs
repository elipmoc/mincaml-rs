use crate::util::*;

named!(comment_begin_parser<()>,do_parse!(
    tag!("(*")>>(())
));

named!(comment_end_parser<()>,do_parse!(
    tag!("*)")>>(())
));

named!(comment_parser<()>,do_parse!(
    comment_begin_parser
    >> many0!(
        alt!(
           comment_parser |
           map!(preceded!(not!(comment_end_parser),take!(1)),|_|(()))
        )
       )
    >> comment_end_parser
    >> (())
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
    )
    >>(bool)
));

named!(not_parser<()>,do_parse!(
    tag!("not")>>(())
));

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
