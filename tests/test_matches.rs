use grep_starter_rust::*;

fn assert_match(input: &str, pattern: &str, from: usize, to: usize) {
    assert_eq!(match_pattern(input, pattern), Some((from, to)));
}

fn assert_not_match(input: &str, pattern: &str) {
    assert_eq!(match_pattern(input, pattern), None);
}

#[test]
fn test_match_text() {
    assert_match("hey ! hello world", "hello", 6, 11);
    assert_not_match("Yeah", "hello");
}

#[test]
fn test_match_digit() {
    assert_match("hey89world", r"\d", 3, 4);
    assert_not_match("Yeah", r"\d");
}

#[test]
fn test_match_chars() {
    assert_match("alpha-num3ric", r"\w", 0, 1);
    assert_match("foo101", r"\w", 0, 1);
    assert_not_match("$!?", r"\w");
}

#[test]
fn test_match_pos_chars_group() {
    assert_match("apple", r"[abc]", 0, 1);
    assert_not_match("dog", r"[abc]");
}

#[test]
fn test_match_neg_chars_group() {
    assert_match("dog", r"[^abc]", 0, 1);
    assert_not_match("cab", r"[^abc]");
}

#[test]
fn test_match_seq() {
    assert_match("100 apples", r"\d\d\d apple", 0, 9);
    assert_not_match("1 apple", r"\d\d\d apple");
    assert_not_match("cab", r"\d\d\d apple");
}

#[test]
fn test_start_string() {
    assert_match("log", "^log", 0, 3);
    assert_not_match("slog", "^log");
}

#[test]
fn test_end_string() {
    assert_match("dog", "dog$", 0, 3);
    assert_not_match("dogs", "dog$");
}

#[test]
fn test_one_or_more() {
    assert_match("apple", "a+", 0, 1);
    assert_match("SaaS", "a+", 1, 3);
    assert_match("apple", "a+pple", 0, 5);
    assert_not_match("ale", "a+p+le");
    assert_not_match("ple", "a+p+le");
    assert_match("aple", "a+p+le", 0, 4);
    assert_match("aaaapppppple", "a+p+le", 0, 12);
    assert_match("aaaapppppple", "^a+p+le$", 0, 12);
    assert_not_match("appLE", "a+pple");
    assert_match("SaaS", "^Sa+S$", 0, 4);
    assert_not_match("dogs", "a+");
    assert_match("dogs", "d.+gs$", 0, 4);
    assert_match(" cat and cat", r"\w+ and \w+", 1, 12);
}

#[test]
fn test_zero_or_one() {
    assert_match("dog", "dogs?", 0, 3);
    assert_match("dogs", "dogs?", 0, 4);
    assert_match("dogy", "dogs?", 0, 3);
    assert_match("dogy", "dogs?y", 0, 4);
    assert_match("dogsy", "dogs?y", 0, 5);
    assert_not_match("dogoy", "dogs?y");
    assert_match("dog", "^dogs?$", 0, 3);
    assert_match("dogs", "^dogs?$", 0, 4);
}

#[test]
fn test_wildcard() {
    assert_match("dog", "d.g", 0, 3);
    assert_match("dog", "d.g", 0, 3);
    assert_not_match("dg", "d.g");
    assert_match("dg", "d.?g", 0, 2);
    assert_match("dogs", "^d.?gs?$", 0, 4);
    assert_match("dags", "^d.?gs?$", 0, 4);
    assert_match("dg", "^d.?gs?$", 0, 2);
    assert_match("dig", "^d.?gs?$", 0, 3);
    assert_match("digs", "^d.?gs?$", 0, 4);
    assert_not_match("digsi", "^d.?gs?$");
    assert_not_match("digis", "^d.?gs?$");
    assert_not_match("diig", "^d.?gs?$");
}

#[test]
fn test_alternation() {
    assert_match("cat", "(cat|dog)", 0, 3);
    assert_match("  cat  ", "(cat)", 2, 5);
    assert_match("dog", "(cat|dog)", 0, 3);
    assert_match("dog", "(dog)", 0, 3);
    assert_not_match("dig", "(cat|dog)");
    assert_not_match("dog", "(cat|dog)s");
    assert_match(" dogs", "(some|more)? (cat|dog)s", 0, 5);
    assert_match("some dogs", "(some|more) (cat|dog)s", 0, 9);
    assert_not_match("more cat", "(some|more) (cat|dog)s");
    assert_match("more cat", "(some|more) (cat|dog)s?", 0, 8);
    assert_not_match("none dogs", "(some|more) (cat|dog)s");
    assert_match(" cat and cat", r"(cat) and (cat)", 1, 12);
    assert_match(" cat and cat", r"(\w+) and (\w+)", 1, 12);
}

#[test]
fn test_back_reference() {
    assert_not_match("", r"(cat) and \1");
    assert_match(" cat and cat ", r"(cat) and \1", 1, 12);
    assert_not_match(" cat and dog ", r"(cat) and \1");
    assert_match(" cat and cat ", r"(\w+) and \1", 1, 12);
    assert_match(" dog and dog ", r"(\w+) and \1", 1, 12);
    assert_not_match(" cat and dog ", r"(\w+) and \1");
}

#[test]
fn test_multi_back_reference() {
    assert_match(
        " 3 red squares and 3 red circles ",
        r"(\d+) (\w+) squares and \1 \2 circles",
        1,
        32,
    );
    assert_not_match(
        "3 red squares and 4 red circles",
        r"(\d+) (\w+) squares and \1 \2 circles",
    );
}

#[test]
fn test_nested_back_reference() {
    assert_match(
        " 'cat and cat' is the same as 'cat and cat' ",
        r"('(cat) and \2') is the same as \1",
        1,
        43,
    );
    assert_match(
        "grep 101 is doing grep 101 times, and again grep 101 times",
        r"((\w\w\w\w) (\d\d\d)) is doing \2 \3 times, and again \1 times",
        0,
        58,
    );
}
