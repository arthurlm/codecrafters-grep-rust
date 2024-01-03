mod error;

pub use error::*;

pub type MatchResult = (usize, usize);

pub fn match_pattern(input_line: &str, input_pattern: &str) -> Option<MatchResult> {
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

    pub fn matches(&self, input_line: &str) -> Option<MatchResult> {
        if self.patterns.first() == Some(&Pattern::Start) {
            match_here(&self.patterns[1..], MatchContext::new(0, input_line))
        } else {
            for start_idx in 0..input_line.len() {
                if let Some(res) =
                    match_here(&self.patterns, MatchContext::new(start_idx, input_line))
                {
                    return Some(res);
                }
            }

            None
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

fn match_here(patterns: &[Pattern], context: MatchContext) -> Option<MatchResult> {
    match (context.first_char(), patterns.split_first()) {
        // Check if pattern and current char match
        (Some(input_char), Some((Pattern::Literal(char), rem_patterns))) if input_char == *char => {
            match_here(rem_patterns, context.next_char())
        }
        (Some(input_char), Some((Pattern::Digit, rem_patterns))) if input_char.is_ascii_digit() => {
            match_here(rem_patterns, context.next_char())
        }
        (Some(input_char), Some((Pattern::Chars, rem_patterns)))
            if input_char.is_alphanumeric() =>
        {
            match_here(rem_patterns, context.next_char())
        }
        (Some(input_char), Some((Pattern::PositiveCharGroup(values), rem_patterns)))
            if values.contains(&input_char) =>
        {
            match_here(rem_patterns, context.next_char())
        }
        (Some(input_char), Some((Pattern::NegativeCharGroup(values), rem_patterns)))
            if !values.contains(&input_char) =>
        {
            match_here(rem_patterns, context.next_char())
        }
        (Some(_), Some((Pattern::Wildcard, rem_patterns))) => {
            match_here(rem_patterns, context.next_char())
        }
        (_, Some((Pattern::OneOrMore(pattern), rem_patterns))) => {
            // Match inner pattern with remaining patterns = match one
            match_here(&concat_pattern(pattern, rem_patterns), context.clone()).or(
                // Or match inner pattern with input patterns = match more
                match_here(&concat_pattern(pattern, patterns), context.clone()),
            )
        }
        (_, Some((Pattern::ZeroOrOne(pattern), rem_patterns))) => {
            // Match one
            match_here(&concat_pattern(pattern, rem_patterns), context.clone()).or(
                // Match zero
                match_here(rem_patterns, context),
            )
        }
        (_, Some((Pattern::Alternation(alternations), rem_patterns))) => {
            for alt in alternations {
                if let Some(res) = match_here(&concat_patterns(alt, rem_patterns), context.clone())
                {
                    return Some(res);
                }
            }

            None
        }
        // Check end pattern
        (None, Some((Pattern::End, _rem_patterns))) => {
            Some((context.start_index, context.current_index))
        }
        // If there is no more pattern
        (_, None) => Some((context.start_index, context.current_index)),
        // // It there is some pattern left and it did not match whatever char we have
        (_, Some(_)) => None,
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

#[derive(Debug, Clone, Default)]
pub struct MatchContext<'a> {
    start_index: usize,
    current_index: usize,
    input_line: &'a str,
}

impl<'a> MatchContext<'a> {
    pub fn new(start_index: usize, input_line: &'a str) -> Self {
        Self {
            start_index,
            current_index: start_index,
            input_line,
        }
    }

    pub fn next_char(&self) -> Self {
        Self {
            start_index: self.start_index,
            current_index: self.current_index + 1,
            input_line: self.input_line,
        }
    }

    pub fn first_char(&self) -> Option<char> {
        self.input_line.chars().nth(self.current_index)
    }
}
