mod error;

pub use error::*;

pub fn match_pattern(input_line: &str, input_pattern: &str) -> bool {
    let re = Regexp::parse(input_pattern).expect("Unhandled pattern");
    re.matches(input_line)
}

#[derive(Debug, PartialEq, Eq)]
pub struct Regexp {
    pub patterns: Vec<Pattern>,
    pub start_string_anchor: bool,
    pub end_string_anchor: bool,
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

        // Parse anchor
        let start_string_anchor = if let Some(next_input) = input.strip_prefix('^') {
            input = next_input;
            true
        } else {
            false
        };

        let end_string_anchor = if let Some(next_input) = input.strip_suffix('$') {
            input = next_input;
            true
        } else {
            false
        };

        // Parse pattern
        loop {
            let (next_input, pattern) = Pattern::parse(input)?;
            patterns.push(pattern);
            if next_input.is_empty() {
                break;
            }
            input = next_input;
        }

        // Build output
        Ok(Self {
            patterns,
            start_string_anchor,
            end_string_anchor,
        })
    }

    fn match_at_beginning(&self, input_lines: &str) -> Option<usize> {
        let input_chars: Vec<_> = input_lines.chars().collect();

        let mut input_idx = 0;
        let mut pattern_idx = 0;

        loop {
            match (input_chars.get(input_idx), self.patterns.get(pattern_idx)) {
                // Check if pattern and current char match
                (Some(c), Some(p)) if p.matches(*c) => {
                    input_idx += 1;
                    pattern_idx += 1;
                }
                // If there is no more pattern
                (_, None) => return Some(input_idx),
                // It there is some pattern left and it did not match whatever char we have
                (_, Some(_)) => return None,
            }
        }
    }

    pub fn matches(&self, input_line: &str) -> bool {
        fn match_end(this: &Regexp, end_index: Option<usize>, input_len: usize) -> bool {
            if this.end_string_anchor {
                matches!(end_index, Some(idx) if idx == input_len)
            } else {
                end_index.is_some()
            }
        }

        if self.start_string_anchor {
            match_end(&self, self.match_at_beginning(input_line), input_line.len())
        } else {
            for start_idx in 0..input_line.len() {
                if match_end(
                    &self,
                    self.match_at_beginning(&input_line[start_idx..]),
                    input_line.len(),
                ) {
                    return true;
                }
            }

            false
        }
    }
}

impl Pattern {
    pub fn parse(input: &str) -> Result<(&str, Self), GrepError> {
        if let Some(input) = input.strip_prefix(r"\d") {
            Ok((input, Self::Digit))
        } else if let Some(input) = input.strip_prefix(r"\w") {
            Ok((input, Self::Chars))
        } else if let Some(input) = input.strip_prefix('[') {
            match input.chars().position(|c| c == ']') {
                None => Err(GrepError::InvalidPattern),
                Some(end) => {
                    let sub_input = &input[..end];
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
