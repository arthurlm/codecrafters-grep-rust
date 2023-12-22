use grep_starter_rust::*;

#[test]
fn test_match_text() {
    let r = Regexp {
        patterns: vec![
            Pattern::Literal('h'),
            Pattern::Literal('e'),
            Pattern::Literal('l'),
            Pattern::Literal('l'),
            Pattern::Literal('o'),
        ],
        start_string_anchor: false,
    };
    assert!(r.matches("hey ! hello world"));
    assert!(!r.matches("Yeah"));
}

#[test]
fn test_match_digit() {
    let re = Regexp {
        patterns: vec![Pattern::Digit],
        start_string_anchor: false,
    };
    assert!(re.matches("hey89world"));
    assert!(!re.matches("Yeah"));
}

#[test]
fn test_match_chars() {
    let re = Regexp {
        patterns: vec![Pattern::Chars],
        start_string_anchor: false,
    };
    assert!(re.matches("alpha-num3ric"));
    assert!(re.matches("foo101"));
    assert!(!re.matches("$!?"));
}

#[test]
fn test_match_pos_chars_group() {
    let re = Regexp {
        patterns: vec![Pattern::PositiveCharGroup(vec!['a', 'b', 'c'])],
        start_string_anchor: false,
    };
    assert!(re.matches("apple"));
    assert!(!re.matches("dog"));
}

#[test]
fn test_match_neg_chars_group() {
    let re = Regexp {
        patterns: vec![Pattern::NegativeCharGroup(vec!['a', 'b', 'c'])],
        start_string_anchor: false,
    };
    assert!(re.matches("dog"));
    assert!(!re.matches("cab"));
}

#[test]
fn test_match_seq() {
    let re = Regexp {
        patterns: vec![
            Pattern::Digit,
            Pattern::Digit,
            Pattern::Digit,
            Pattern::Literal(' '),
            Pattern::Literal('a'),
            Pattern::Literal('p'),
            Pattern::Literal('p'),
            Pattern::Literal('l'),
            Pattern::Literal('e'),
        ],
        start_string_anchor: false,
    };

    assert!(re.matches("100 apples"));
    assert!(!re.matches("1 apple"));
    assert!(!re.matches("cab"));
}

#[test]
fn test_start_string() {
    assert!(match_pattern("log", "^log"));
    assert!(!match_pattern("slog", "^log"));
}

#[test]
fn test_end_string() {
    assert!(match_pattern("dog", "dog$"));
    assert!(!match_pattern("dogs", "dog$"));
}
