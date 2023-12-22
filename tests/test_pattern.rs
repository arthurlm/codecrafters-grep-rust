use grep_starter_rust::{GrepError, Pattern};

#[test]
fn test_parse_pattern() {
    let (r, x) = Pattern::parse(r"hello").unwrap();
    assert_eq!(x, Pattern::Text("hello".to_string()));
    assert_eq!(r, "");

    let (r, x) = Pattern::parse(r"\d").unwrap();
    assert_eq!(x, Pattern::Digit);
    assert_eq!(r, "");

    let (r, x) = Pattern::parse(r"\w").unwrap();
    assert_eq!(x, Pattern::Chars);
    assert_eq!(r, "");

    let (r, x) = Pattern::parse(r"[abc]").unwrap();
    assert_eq!(x, Pattern::PositiveCharGroup(vec!['a', 'b', 'c']));
    assert_eq!(r, "");
}

#[test]
fn test_parse_invalid_pattern() {
    let e = Pattern::parse("").unwrap_err();
    assert_eq!(e, GrepError::InvalidPattern);

    let e = Pattern::parse("[abc").unwrap_err();
    assert_eq!(e, GrepError::InvalidPattern);
}
