mod error;

pub use error::*;

pub fn match_pattern(input_line: &str, input_pattern: &str) -> bool {
    let re = Regexp::parse(input_pattern).expect("Unhandled pattern");
    re.matches(input_line)
}

#[derive(Debug, PartialEq, Eq)]
pub struct Regexp {
    pub patterns: Vec<Pattern>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Pattern {
    Literal(char),
    Digit,
    Chars,
    PositiveCharGroup(Vec<char>),
    NegativeCharGroup(Vec<char>),
    Start,
    End,
    OneOrMore(Box<Pattern>),
    ZeroOrOne(Box<Pattern>),
    Wildcard,
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
            if let Some(next_input) = input.strip_prefix('+') {
                input = next_input;
                let prev = patterns.pop().ok_or(GrepError::InvalidPattern)?;
                patterns.push(Pattern::OneOrMore(Box::new(prev)));
            }

            if let Some(next_input) = input.strip_prefix('?') {
                input = next_input;
                let prev = patterns.pop().ok_or(GrepError::InvalidPattern)?;
                patterns.push(Pattern::ZeroOrOne(Box::new(prev)));
            }

            if input.is_empty() {
                break;
            }

            let (next_input, pattern) = Pattern::parse(input)?;
            patterns.push(pattern);
            input = next_input;
        }

        if patterns.is_empty() {
            return Err(GrepError::InvalidPattern);
        }

        // Build output
        if start_string_anchor {
            patterns.insert(0, Pattern::Start);
        }

        if end_string_anchor {
            patterns.push(Pattern::End);
        }

        Ok(Self { patterns })
    }

    pub fn matches(&self, input_line: &str) -> bool {
        if self.patterns.first() == Some(&Pattern::Start) {
            match_here(&self.patterns[1..], input_line)
        } else {
            for start_idx in 0..input_line.len() {
                if match_here(&self.patterns, &input_line[start_idx..]) {
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
        } else if let Some(input) = input.strip_prefix('.') {
            Ok((input, Self::Wildcard))
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
}

fn match_here(patterns: &[Pattern], input_lines: &str) -> bool {
    match (input_lines.chars().next(), patterns.first()) {
        // Check if pattern and current char match
        (Some(input_char), Some(Pattern::Literal(char))) if input_char == *char => {
            match_here(&patterns[1..], &input_lines[1..])
        }
        (Some(input_char), Some(Pattern::Digit)) if input_char.is_ascii_digit() => {
            match_here(&patterns[1..], &input_lines[1..])
        }
        (Some(input_char), Some(Pattern::Chars)) if input_char.is_alphanumeric() => {
            match_here(&patterns[1..], &input_lines[1..])
        }
        (Some(input_char), Some(Pattern::PositiveCharGroup(values)))
            if values.contains(&input_char) =>
        {
            match_here(&patterns[1..], &input_lines[1..])
        }
        (Some(input_char), Some(Pattern::NegativeCharGroup(values)))
            if !values.contains(&input_char) =>
        {
            match_here(&patterns[1..], &input_lines[1..])
        }
        (Some(_), Some(Pattern::Wildcard)) => match_here(&patterns[1..], &input_lines[1..]),
        (_, Some(Pattern::OneOrMore(pattern))) => {
            let mut count = 0;

            loop {
                if match_here(&[pattern.as_ref().clone()], &input_lines[count..]) {
                    count += 1;
                } else {
                    break;
                }
            }

            if count > 0 {
                match_here(&patterns[1..], &input_lines[count..])
            } else {
                false
            }
        }
        (_, Some(Pattern::ZeroOrOne(pattern))) => {
            let mut next_patterns = patterns.to_vec();
            next_patterns.insert(0, pattern.as_ref().clone());

            match_here(&next_patterns, input_lines) | match_here(&patterns[1..], input_lines)
        }
        // Check end pattern
        (None, Some(Pattern::End)) => true,
        // If there is no more pattern
        (_, None) => true,
        // // It there is some pattern left and it did not match whatever char we have
        (_, Some(_)) => false,
    }
}
