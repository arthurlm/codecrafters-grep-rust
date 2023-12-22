use grep_starter_rust::{GrepError, Pattern};

#[test]
fn test_parse_pattern() {
    let x: Pattern = "hello".parse().unwrap();
    assert_eq!(x, Pattern::Text("hello".to_string()));

    let x: Pattern = r"\d".parse().unwrap();
    assert_eq!(x, Pattern::Digit);
}

#[test]
fn test_parse_invalid_pattern() {
    let x = "".parse::<Pattern>().unwrap_err();
    assert_eq!(x, GrepError::InvalidPattern);
}
