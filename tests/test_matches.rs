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
    };
    assert!(r.matches("hey ! hello world"));
    assert!(!r.matches("Yeah"));
}

#[test]
fn test_match_digit() {
    let re = Regexp {
        patterns: vec![Pattern::Digit],
    };
    assert!(re.matches("hey89world"));
    assert!(!re.matches("Yeah"));
}

#[test]
fn test_match_chars() {
    let re = Regexp {
        patterns: vec![Pattern::Chars],
    };
    assert!(re.matches("alpha-num3ric"));
    assert!(re.matches("foo101"));
    assert!(!re.matches("$!?"));
}

#[test]
fn test_match_pos_chars_group() {
    let re = Regexp {
        patterns: vec![Pattern::PositiveCharGroup(vec!['a', 'b', 'c'])],
    };
    assert!(re.matches("apple"));
    assert!(!re.matches("dog"));
}

#[test]
fn test_match_neg_chars_group() {
    let re = Regexp {
        patterns: vec![Pattern::NegativeCharGroup(vec!['a', 'b', 'c'])],
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

#[test]
fn test_one_or_more() {
    assert!(match_pattern("apple", "a+"));
    assert!(match_pattern("SaaS", "a+"));
    assert!(match_pattern("apple", "a+pple"));
    assert!(!match_pattern("ale", "a+p+le"));
    assert!(!match_pattern("ple", "a+p+le"));
    assert!(match_pattern("aple", "a+p+le"));
    assert!(match_pattern("aaaapppppple", "a+p+le"));
    assert!(match_pattern("aaaapppppple", "^a+p+le$"));
    assert!(!match_pattern("appLE", "a+pple"));
    assert!(match_pattern("SaaS", "^Sa+S$"));
    assert!(!match_pattern("dogs", "a+"));
}

#[test]
fn test_zero_or_one() {
    assert!(match_pattern("dog", "dogs?"));
    assert!(match_pattern("dogs", "dogs?"));
    assert!(match_pattern("dogy", "dogs?"));
    assert!(match_pattern("dogy", "dogs?y"));
    assert!(match_pattern("dogsy", "dogs?y"));
    assert!(!match_pattern("dogoy", "dogs?y"));
    assert!(match_pattern("dog", "^dogs?$"));
    assert!(match_pattern("dogs", "^dogs?$"));
}
