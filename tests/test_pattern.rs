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
        }
    );

    assert_eq!(
        Regexp::parse(r"\d").unwrap(),
        Regexp {
            patterns: vec![Pattern::Digit],
        }
    );

    assert_eq!(
        Regexp::parse(r"\w").unwrap(),
        Regexp {
            patterns: vec![Pattern::Chars],
        }
    );

    assert_eq!(
        Regexp::parse(r"[abc]").unwrap(),
        Regexp {
            patterns: vec![Pattern::PositiveCharGroup(vec!['a', 'b', 'c'])],
        }
    );

    assert_eq!(
        Regexp::parse(r"[^defg]").unwrap(),
        Regexp {
            patterns: vec![Pattern::NegativeCharGroup(vec!['d', 'e', 'f', 'g'])],
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
        }
    );

    assert_eq!(
        Regexp::parse(r"^\dd").unwrap(),
        Regexp {
            patterns: vec![Pattern::Start, Pattern::Digit, Pattern::Literal('d')],
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
        }
    );

    assert_eq!(
        Regexp::parse(r"\dd$").unwrap(),
        Regexp {
            patterns: vec![Pattern::Digit, Pattern::Literal('d'), Pattern::End],
        }
    );

    assert_eq!(
        Regexp::parse(r"\w+").unwrap(),
        Regexp {
            patterns: vec![Pattern::OneOrMore(Box::new(Pattern::Chars))],
        }
    );

    assert_eq!(
        Regexp::parse(r"xx+x").unwrap(),
        Regexp {
            patterns: vec![
                Pattern::Literal('x'),
                Pattern::OneOrMore(Box::new(Pattern::Literal('x'))),
                Pattern::Literal('x')
            ],
        }
    );

    assert_eq!(
        Regexp::parse(r"^x[aze]+").unwrap(),
        Regexp {
            patterns: vec![
                Pattern::Start,
                Pattern::Literal('x'),
                Pattern::OneOrMore(Box::new(Pattern::PositiveCharGroup(vec!['a', 'z', 'e'])))
            ],
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
