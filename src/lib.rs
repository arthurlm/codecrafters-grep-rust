mod error;

pub use error::*;

#[derive(Debug, PartialEq, Eq)]
pub struct Regexp {
    pub patterns: Vec<Pattern>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Pattern {
    Literal(char),
    Digit,
    Chars,
    PositiveCharGroup(Vec<char>),
    NegativeCharGroup(Vec<char>),
}

impl Regexp {
    pub fn parse(mut input: &str) -> Result<Self, GrepError> {
        let mut patterns = Vec::new();

        loop {
            let (next_input, pattern) = Pattern::parse(input)?;
            patterns.push(pattern);
            if next_input.is_empty() {
                break;
            }
            input = next_input;
        }

        Ok(Self { patterns })
    }

    fn matches_start(&self, input_lines: &str) -> bool {
        for (idx, pattern) in self.patterns.iter().enumerate() {
            let Some(c) = input_lines.chars().nth(idx) else {
                return false;
            };

            if !pattern.matches(c) {
                return false;
            }
        }

        true
    }

    pub fn matches(&self, input_lines: &str) -> bool {
        for start_idx in 0..input_lines.len() {
            if self.matches_start(&input_lines[start_idx..]) {
                return true;
            }
        }

        false
    }
}

impl Pattern {
    pub fn parse(input: &str) -> Result<(&str, Self), GrepError> {
        if let Some(new_input) = input.strip_prefix(r"\d") {
            Ok((new_input, Self::Digit))
        } else if let Some(new_input) = input.strip_prefix(r"\w") {
            Ok((new_input, Self::Chars))
        } else if input.starts_with('[') {
            match input.chars().position(|c| c == ']') {
                None => Err(GrepError::InvalidPattern),
                Some(end) => {
                    let sub_input = &input[1..end];
                    let mut chars: Vec<_> = sub_input.chars().collect();
                    if chars.first().copied() == Some('^') {
                        Ok((
                            &input[end + 1..],
                            Self::NegativeCharGroup(chars.drain(1..).collect()),
                        ))
                    } else {
                        Ok((&input[end + 1..], Self::PositiveCharGroup(chars)))
                    }
                }
            }
        } else if input.is_empty() {
            Err(GrepError::InvalidPattern)
        } else {
            let (val, input) = input.split_at(1);
            assert_eq!(val.len(), 1);

            Ok((
                input,
                Self::Literal(val.chars().next().expect("split at fail")),
            ))
        }
    }

    pub fn matches(&self, input_char: char) -> bool {
        match self {
            Self::Literal(char) => input_char == *char,
            Self::Digit => input_char.is_ascii_digit(),
            Self::Chars => input_char.is_alphanumeric(),
            Self::PositiveCharGroup(values) => values.contains(&input_char),
            Self::NegativeCharGroup(values) => !values.contains(&input_char),
        }
    }
}
