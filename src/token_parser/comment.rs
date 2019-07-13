named!(comment_begin_parser<()>,do_parse!(
    tag!("(*")>>(())
));

named!(comment_end_parser<()>,do_parse!(
    tag!("*)")>>(())
));

named!(pub comment_parser<()>,do_parse!(
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