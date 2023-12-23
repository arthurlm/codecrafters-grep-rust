use grep_starter_rust::*;

#[test]
fn test_match_text() {
    assert!(match_pattern("hey ! hello world", "hello"));
    assert!(!match_pattern("Yeah", "hello"));
}

#[test]
fn test_match_digit() {
    assert!(match_pattern("hey89world", r"\d"));
    assert!(!match_pattern("Yeah", r"\d"));
}

#[test]
fn test_match_chars() {
    assert!(match_pattern("alpha-num3ric", r"\w"));
    assert!(match_pattern("foo101", r"\w"));
    assert!(!match_pattern("$!?", r"\w"));
}

#[test]
fn test_match_pos_chars_group() {
    assert!(match_pattern("apple", r"[abc]"));
    assert!(!match_pattern("dog", r"[abc]"));
}

#[test]
fn test_match_neg_chars_group() {
    assert!(match_pattern("dog", r"[^abc]"));
    assert!(!match_pattern("cab", r"[^abc]"));
}

#[test]
fn test_match_seq() {
    assert!(match_pattern("100 apples", r"\d\d\d apple"));
    assert!(!match_pattern("1 apple", r"\d\d\d apple"));
    assert!(!match_pattern("cab", r"\d\d\d apple"));
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
    assert!(match_pattern("dogs", "d.+gs$"));
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

#[test]
fn test_wildcard() {
    assert!(match_pattern("dog", "d.g"));
    assert!(match_pattern("dog", "d.g"));
    assert!(!match_pattern("dg", "d.g"));
    assert!(match_pattern("dg", "d.?g"));
    assert!(match_pattern("dogs", "^d.?gs?$"));
    assert!(match_pattern("dags", "^d.?gs?$"));
    assert!(match_pattern("dg", "^d.?gs?$"));
    assert!(match_pattern("dig", "^d.?gs?$"));
    assert!(match_pattern("digs", "^d.?gs?$"));
    assert!(!match_pattern("digsi", "^d.?gs?$"));
    assert!(!match_pattern("digis", "^d.?gs?$"));
    assert!(!match_pattern("diig", "^d.?gs?$"));
}

#[test]
fn test_alternation() {
    assert!(match_pattern("cat", "(cat|dog)"));
    assert!(match_pattern("dog", "(cat|dog)"));
    assert!(!match_pattern("dig", "(cat|dog)"));
    // assert!(!match_pattern("dog", "(cat|dog)s"));
}
