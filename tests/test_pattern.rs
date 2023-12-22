use grep_starter_rust::*;

#[test]
fn test_parse_pattern() {
    assert_eq!(
        Regexp::parse(r"hello").unwrap(),
        Regexp {
            patterns: vec![
                Pattern::Literal('h'),
                Pattern::Literal('e'),
                Pattern::Literal('l'),
                Pattern::Literal('l'),
                Pattern::Literal('o'),
            ],
            start_string_anchor: false,
            end_string_anchor: false,
        }
    );

    assert_eq!(
        Regexp::parse(r"\d").unwrap(),
        Regexp {
            patterns: vec![Pattern::Digit],
            start_string_anchor: false,
            end_string_anchor: false,
        }
    );

    assert_eq!(
        Regexp::parse(r"\w").unwrap(),
        Regexp {
            patterns: vec![Pattern::Chars],
            start_string_anchor: false,
            end_string_anchor: false,
        }
    );

    assert_eq!(
        Regexp::parse(r"[abc]").unwrap(),
        Regexp {
            patterns: vec![Pattern::PositiveCharGroup(vec!['a', 'b', 'c'])],
            start_string_anchor: false,
            end_string_anchor: false,
        }
    );

    assert_eq!(
        Regexp::parse(r"[^defg]").unwrap(),
        Regexp {
            patterns: vec![Pattern::NegativeCharGroup(vec!['d', 'e', 'f', 'g'])],
            start_string_anchor: false,
            end_string_anchor: false,
        }
    );

    assert_eq!(
        Regexp::parse(r"\d apple").unwrap(),
        Regexp {
            patterns: vec![
                Pattern::Digit,
                Pattern::Literal(' '),
                Pattern::Literal('a'),
                Pattern::Literal('p'),
                Pattern::Literal('p'),
                Pattern::Literal('l'),
                Pattern::Literal('e'),
            ],
            start_string_anchor: false,
            end_string_anchor: false,
        }
    );

    assert_eq!(
        Regexp::parse(r"\d \d ap[plx]le").unwrap(),
        Regexp {
            patterns: vec![
                Pattern::Digit,
                Pattern::Literal(' '),
                Pattern::Digit,
                Pattern::Literal(' '),
                Pattern::Literal('a'),
                Pattern::Literal('p'),
                Pattern::PositiveCharGroup(vec!['p', 'l', 'x']),
                Pattern::Literal('l'),
                Pattern::Literal('e'),
            ],
            start_string_anchor: false,
            end_string_anchor: false,
        }
    );

    assert_eq!(
        Regexp::parse(r"d^d").unwrap(),
        Regexp {
            patterns: vec![
                Pattern::Literal('d'),
                Pattern::Literal('^'),
                Pattern::Literal('d'),
            ],
            start_string_anchor: false,
            end_string_anchor: false,
        }
    );

    assert_eq!(
        Regexp::parse(r"^\dd").unwrap(),
        Regexp {
            patterns: vec![Pattern::Digit, Pattern::Literal('d')],
            start_string_anchor: true,
            end_string_anchor: false,
        }
    );

    assert_eq!(
        Regexp::parse(r"d$d").unwrap(),
        Regexp {
            patterns: vec![
                Pattern::Literal('d'),
                Pattern::Literal('$'),
                Pattern::Literal('d'),
            ],
            start_string_anchor: false,
            end_string_anchor: false,
        }
    );

    assert_eq!(
        Regexp::parse(r"\dd$").unwrap(),
        Regexp {
            patterns: vec![Pattern::Digit, Pattern::Literal('d')],
            start_string_anchor: false,
            end_string_anchor: true,
        }
    );
}

#[test]
fn test_parse_invalid_pattern() {
    let e = Regexp::parse("").unwrap_err();
    assert_eq!(e, GrepError::InvalidPattern);

    let e = Regexp::parse("[abc").unwrap_err();
    assert_eq!(e, GrepError::InvalidPattern);
}
