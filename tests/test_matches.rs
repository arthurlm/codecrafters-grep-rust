use grep_starter_rust::*;

#[test]
fn test_match_text() {
    assert_eq!(match_pattern("hey ! hello world", "hello"), Some((6, 11)));
    assert_eq!(match_pattern("Yeah", "hello"), None);
}

#[test]
fn test_match_digit() {
    assert_eq!(match_pattern("hey89world", r"\d"), Some((3, 4)));
    assert_eq!(match_pattern("Yeah", r"\d"), None);
}

#[test]
fn test_match_chars() {
    assert_eq!(match_pattern("alpha-num3ric", r"\w"), Some((0, 1)));
    assert_eq!(match_pattern("foo101", r"\w"), Some((0, 1)));
    assert_eq!(match_pattern("$!?", r"\w"), None);
}

#[test]
fn test_match_pos_chars_group() {
    assert_eq!(match_pattern("apple", r"[abc]"), Some((0, 1)));
    assert_eq!(match_pattern("dog", r"[abc]"), None);
}

#[test]
fn test_match_neg_chars_group() {
    assert_eq!(match_pattern("dog", r"[^abc]"), Some((0, 1)));
    assert_eq!(match_pattern("cab", r"[^abc]"), None);
}

#[test]
fn test_match_seq() {
    assert_eq!(match_pattern("100 apples", r"\d\d\d apple"), Some((0, 9)));
    assert_eq!(match_pattern("1 apple", r"\d\d\d apple"), None);
    assert_eq!(match_pattern("cab", r"\d\d\d apple"), None);
}

#[test]
fn test_start_string() {
    assert_eq!(match_pattern("log", "^log"), Some((0, 3)));
    assert_eq!(match_pattern("slog", "^log"), None);
}

#[test]
fn test_end_string() {
    assert_eq!(match_pattern("dog", "dog$"), Some((0, 3)));
    assert_eq!(match_pattern("dogs", "dog$"), None);
}

#[test]
fn test_one_or_more() {
    assert_eq!(match_pattern("apple", "a+"), Some((0, 1)));
    assert_eq!(match_pattern("SaaS", "a+"), Some((1, 2)));
    assert_eq!(match_pattern("apple", "a+pple"), Some((0, 5)));
    assert_eq!(match_pattern("ale", "a+p+le"), None);
    assert_eq!(match_pattern("ple", "a+p+le"), None);
    assert_eq!(match_pattern("aple", "a+p+le"), Some((0, 4)));
    assert_eq!(match_pattern("aaaapppppple", "a+p+le"), Some((0, 12)));
    assert_eq!(match_pattern("aaaapppppple", "^a+p+le$"), Some((0, 12)));
    assert_eq!(match_pattern("appLE", "a+pple"), None);
    assert_eq!(match_pattern("SaaS", "^Sa+S$"), Some((0, 4)));
    assert_eq!(match_pattern("dogs", "a+"), None);
    assert_eq!(match_pattern("dogs", "d.+gs$"), Some((0, 4)));
}

#[test]
fn test_zero_or_one() {
    assert_eq!(match_pattern("dog", "dogs?"), Some((0, 3)));
    assert_eq!(match_pattern("dogs", "dogs?"), Some((0, 4)));
    assert_eq!(match_pattern("dogy", "dogs?"), Some((0, 3)));
    assert_eq!(match_pattern("dogy", "dogs?y"), Some((0, 4)));
    assert_eq!(match_pattern("dogsy", "dogs?y"), Some((0, 5)));
    assert_eq!(match_pattern("dogoy", "dogs?y"), None);
    assert_eq!(match_pattern("dog", "^dogs?$"), Some((0, 3)));
    assert_eq!(match_pattern("dogs", "^dogs?$"), Some((0, 4)));
}

#[test]
fn test_wildcard() {
    assert_eq!(match_pattern("dog", "d.g"), Some((0, 3)));
    assert_eq!(match_pattern("dog", "d.g"), Some((0, 3)));
    assert_eq!(match_pattern("dg", "d.g"), None);
    assert_eq!(match_pattern("dg", "d.?g"), Some((0, 2)));
    assert_eq!(match_pattern("dogs", "^d.?gs?$"), Some((0, 4)));
    assert_eq!(match_pattern("dags", "^d.?gs?$"), Some((0, 4)));
    assert_eq!(match_pattern("dg", "^d.?gs?$"), Some((0, 2)));
    assert_eq!(match_pattern("dig", "^d.?gs?$"), Some((0, 3)));
    assert_eq!(match_pattern("digs", "^d.?gs?$"), Some((0, 4)));
    assert_eq!(match_pattern("digsi", "^d.?gs?$"), None);
    assert_eq!(match_pattern("digis", "^d.?gs?$"), None);
    assert_eq!(match_pattern("diig", "^d.?gs?$"), None);
}

#[test]
fn test_alternation() {
    assert_eq!(match_pattern("cat", "(cat|dog)"), Some((0, 3)));
    assert_eq!(match_pattern("  cat  ", "(cat)"), Some((2, 5)));
    assert_eq!(match_pattern("dog", "(cat|dog)"), Some((0, 3)));
    assert_eq!(match_pattern("dog", "(dog)"), Some((0, 3)));
    assert_eq!(match_pattern("dig", "(cat|dog)"), None);
    assert_eq!(match_pattern("dog", "(cat|dog)s"), None);
    assert_eq!(
        match_pattern(" dogs", "(some|more)? (cat|dog)s"),
        Some((0, 5))
    );
    assert_eq!(
        match_pattern("some dogs", "(some|more) (cat|dog)s"),
        Some((0, 9))
    );
    assert_eq!(match_pattern("more cat", "(some|more) (cat|dog)s"), None);
    assert_eq!(
        match_pattern("more cat", "(some|more) (cat|dog)s?"),
        Some((0, 8))
    );
    assert_eq!(match_pattern("none dogs", "(some|more) (cat|dog)s"), None);
}
