use grep_starter_rust::Pattern;

#[test]
fn test_match_text() {
    let p = Pattern::Text("hello".to_string());
    assert!(p.matches("hey ! hello world"));
    assert!(!p.matches("Yeah"));
}

#[test]
fn test_match_digit() {
    let p = Pattern::Digit;
    assert!(p.matches("hey89world"));
    assert!(!p.matches("Yeah"));
}

#[test]
fn test_match_chars() {
    let p = Pattern::Chars;
    assert!(p.matches("alpha-num3ric"));
    assert!(p.matches("foo101"));
    assert!(!p.matches("$!?"));
}

#[test]
fn test_match_pos_chars_group() {
    let p = Pattern::PositiveCharGroup(vec!['a', 'b', 'c']);
    assert!(p.matches("apple"));
    assert!(!p.matches("dog"));
}

#[test]
fn test_match_neg_chars_group() {
    let p = Pattern::NegativeCharGroup(vec!['a', 'b', 'c']);
    assert!(p.matches("dog"));
    assert!(!p.matches("cab"));
}
