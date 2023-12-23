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
    Alternation(Vec<Vec<Pattern>>),
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
                    assert!(
                        !sub_input.contains('['),
                        "unsupported nested char groups parse"
                    );

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
        } else if let Some(input) = input.strip_prefix('(') {
            match input.chars().position(|c| c == ')') {
                None => Err(GrepError::InvalidPattern),
                Some(end) => {
                    let sub_input = &input[..end];
                    assert!(
                        !sub_input.contains('('),
                        "unsupported nested alternation parse"
                    );

                    let mut alternations = Vec::new();
                    for mut sub_sequence in sub_input.split('|') {
                        let mut alternation = Vec::new();

                        while !sub_sequence.is_empty() {
                            let (next_sub_sequence, pattern) = Pattern::parse(sub_sequence)?;
                            alternation.push(pattern);
                            sub_sequence = next_sub_sequence;
                        }

                        alternations.push(alternation);
                    }

                    Ok((&input[end + 1..], Self::Alternation(alternations)))
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

fn match_here(patterns: &[Pattern], input_line: &str) -> bool {
    match (input_line.chars().next(), patterns.split_first()) {
        // Check if pattern and current char match
        (Some(input_char), Some((Pattern::Literal(char), rem_patterns))) if input_char == *char => {
            match_here(rem_patterns, &input_line[1..])
        }
        (Some(input_char), Some((Pattern::Digit, rem_patterns))) if input_char.is_ascii_digit() => {
            match_here(rem_patterns, &input_line[1..])
        }
        (Some(input_char), Some((Pattern::Chars, rem_patterns)))
            if input_char.is_alphanumeric() =>
        {
            match_here(rem_patterns, &input_line[1..])
        }
        (Some(input_char), Some((Pattern::PositiveCharGroup(values), rem_patterns)))
            if values.contains(&input_char) =>
        {
            match_here(rem_patterns, &input_line[1..])
        }
        (Some(input_char), Some((Pattern::NegativeCharGroup(values), rem_patterns)))
            if !values.contains(&input_char) =>
        {
            match_here(rem_patterns, &input_line[1..])
        }
        (Some(_), Some((Pattern::Wildcard, rem_patterns))) => {
            match_here(rem_patterns, &input_line[1..])
        }
        (_, Some((Pattern::OneOrMore(pattern), rem_patterns))) => {
            // Match at least once the inner pattern
            if !match_here(&[pattern.as_ref().clone()], input_line) {
                return false;
            }

            // Then continue recursion
            match_here(rem_patterns, &input_line[1..])
            // Or retry again current pattern sequence on next input
            | match_here(patterns, &input_line[1..])
        }
        (_, Some((Pattern::ZeroOrOne(pattern), rem_patterns))) => {
            // Match zero
            match_here(rem_patterns, input_line)
            // Or Match one
            | match_here(&concat_pattern(pattern, rem_patterns), input_line)
        }
        (_, Some((Pattern::Alternation(alternations), rem_patterns))) => alternations
            .iter()
            .any(|alt| match_here(&concat_patterns(alt, rem_patterns), input_line)),
        // Check end pattern
        (None, Some((Pattern::End, _rem_patterns))) => true,
        // If there is no more pattern
        (_, None) => true,
        // // It there is some pattern left and it did not match whatever char we have
        (_, Some(_)) => false,
    }
}

fn concat_pattern(item: &Pattern, items: &[Pattern]) -> Vec<Pattern> {
    let mut output = Vec::with_capacity(items.len() + 1);
    output.push(item.clone());
    output.extend(items.iter().cloned());
    output
}

fn concat_patterns(a: &[Pattern], b: &[Pattern]) -> Vec<Pattern> {
    let mut output = Vec::with_capacity(a.len() + b.len());
    output.extend(a.iter().cloned());
    output.extend(b.iter().cloned());
    output
}
