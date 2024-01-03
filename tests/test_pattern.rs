use grep_starter_rust::*;

#[test]
fn test_debug() {
    assert_eq!(format!("{:?}", Pattern::Chars), "Chars");
    assert_eq!(
        format!(
            "{:?}",
            Regexp {
                patterns: vec![Pattern::Chars]
            }
        ),
        "Regexp { patterns: [Chars] }"
    );
}

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

    assert_eq!(
        Regexp::parse(r"(cat|dog)").unwrap(),
        Regexp {
            patterns: vec![Pattern::Alternation(vec![
                vec![
                    Pattern::Literal('c'),
                    Pattern::Literal('a'),
                    Pattern::Literal('t')
                ],
                vec![
                    Pattern::Literal('d'),
                    Pattern::Literal('o'),
                    Pattern::Literal('g')
                ]
            ])],
        }
    );

    assert_eq!(
        Regexp::parse(r"^\d (cat|dog\d+|duc\w)s?$").unwrap(),
        Regexp {
            patterns: vec![
                Pattern::Start,
                Pattern::Digit,
                Pattern::Literal(' '),
                Pattern::Alternation(vec![
                    vec![
                        Pattern::Literal('c'),
                        Pattern::Literal('a'),
                        Pattern::Literal('t'),
                    ],
                    vec![
                        Pattern::Literal('d'),
                        Pattern::Literal('o'),
                        Pattern::Literal('g'),
                        Pattern::OneOrMore(Box::new(Pattern::Digit)),
                    ],
                    vec![
                        Pattern::Literal('d'),
                        Pattern::Literal('u'),
                        Pattern::Literal('c'),
                        Pattern::Chars,
                    ],
                ]),
                Pattern::ZeroOrOne(Box::new(Pattern::Literal('s'))),
                Pattern::End,
            ],
        }
    );

    assert_eq!(
        Regexp::parse(r"\1").unwrap(),
        Regexp {
            patterns: vec![Pattern::BackReference(0)],
        }
    );

    assert_eq!(
        Regexp::parse(r"('(cat) and \2') is the same as \1").unwrap(),
        Regexp {
            patterns: vec![
                Pattern::Alternation(vec![vec![
                    Pattern::Literal('\''),
                    Pattern::Alternation(vec![vec![
                        Pattern::Literal('c'),
                        Pattern::Literal('a'),
                        Pattern::Literal('t'),
                    ]]),
                    Pattern::Literal(' '),
                    Pattern::Literal('a'),
                    Pattern::Literal('n'),
                    Pattern::Literal('d'),
                    Pattern::Literal(' '),
                    Pattern::BackReference(1),
                    Pattern::Literal('\''),
                ]]),
                Pattern::Literal(' '),
                Pattern::Literal('i'),
                Pattern::Literal('s'),
                Pattern::Literal(' '),
                Pattern::Literal('t'),
                Pattern::Literal('h'),
                Pattern::Literal('e'),
                Pattern::Literal(' '),
                Pattern::Literal('s'),
                Pattern::Literal('a'),
                Pattern::Literal('m'),
                Pattern::Literal('e'),
                Pattern::Literal(' '),
                Pattern::Literal('a'),
                Pattern::Literal('s'),
                Pattern::Literal(' '),
                Pattern::BackReference(0),
            ]
        }
    );

    assert_eq!(
        Regexp::parse(r"((abc|def)|ghi)(jkl|mno|(\w+))(pqr)").unwrap(),
        Regexp {
            patterns: vec![
                Pattern::Alternation(vec![
                    vec![Pattern::Alternation(vec![
                        vec![
                            Pattern::Literal('a'),
                            Pattern::Literal('b'),
                            Pattern::Literal('c'),
                        ],
                        vec![
                            Pattern::Literal('d'),
                            Pattern::Literal('e'),
                            Pattern::Literal('f'),
                        ],
                    ]),],
                    vec![
                        Pattern::Literal('g'),
                        Pattern::Literal('h'),
                        Pattern::Literal('i'),
                    ],
                ]),
                Pattern::Alternation(vec![
                    vec![
                        Pattern::Literal('j'),
                        Pattern::Literal('k'),
                        Pattern::Literal('l'),
                    ],
                    vec![
                        Pattern::Literal('m'),
                        Pattern::Literal('n'),
                        Pattern::Literal('o'),
                    ],
                    vec![Pattern::Alternation(vec![vec![Pattern::OneOrMore(
                        Box::new(Pattern::Chars)
                    )]])]
                ]),
                Pattern::Alternation(vec![vec![
                    Pattern::Literal('p'),
                    Pattern::Literal('q'),
                    Pattern::Literal('r'),
                ],]),
            ]
        }
    );
}

#[test]
fn test_parse_invalid_pattern() {
    let e = Regexp::parse("").unwrap_err();
    assert_eq!(e, GrepError::InvalidPattern);

    let e = Regexp::parse("[abc").unwrap_err();
    assert_eq!(e, GrepError::InvalidPattern);

    let e = Regexp::parse("([abc|[def)").unwrap_err();
    assert_eq!(e, GrepError::InvalidPattern);

    let e = Regexp::parse("+").unwrap_err();
    assert_eq!(e, GrepError::InvalidPattern);

    let e = Regexp::parse("?").unwrap_err();
    assert_eq!(e, GrepError::InvalidPattern);

    let e = Regexp::parse("(abc").unwrap_err();
    assert_eq!(e, GrepError::InvalidPattern);
}

#[test]
#[should_panic]
fn test_valid_unsupported_pattern2() {
    let e = Regexp::parse("[[abc]").unwrap_err();
    assert_eq!(e, GrepError::InvalidPattern);
}
