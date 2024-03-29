mod error;

use std::{
    collections::HashMap,
    sync::atomic::{AtomicUsize, Ordering},
};

pub use error::*;

type MatchResult = (usize, usize);
type ReferenceTable<'a> = HashMap<usize, &'a str>;

pub fn match_pattern(input_line: &str, input_pattern: &str) -> Option<MatchResult> {
    let re = re_parse(input_pattern).expect("Unhandled pattern");
    re.matches(input_line)
}

pub fn re_parse(input_pattern: &str) -> Result<Regexp, GrepError> {
    Regexp::parse(input_pattern, &AtomicUsize::new(1))
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
    Alternation {
        alternations: Vec<Vec<Pattern>>,
        id: usize,
    },
    BackReference(usize),
}

impl Regexp {
    fn parse(mut input: &str, alternation_counter: &AtomicUsize) -> Result<Self, GrepError> {
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

            let (next_input, pattern) = Pattern::parse(input, alternation_counter)?;
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

    fn matches(&self, input_line: &str) -> Option<MatchResult> {
        if self.patterns.first() == Some(&Pattern::Start) {
            if let Some((res, _table)) =
                match_here(&self.patterns[1..], MatchContext::new(0, input_line))
            {
                return Some(res);
            }

            None
        } else {
            for start_idx in 0..input_line.len() {
                if let Some((res, _table)) =
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
    fn parse<'a>(
        input: &'a str,
        alternation_counter: &AtomicUsize,
    ) -> Result<(&'a str, Self), GrepError> {
        if let Some(input) = input.strip_prefix(r"\d") {
            Ok((input, Self::Digit))
        } else if let Some(input) = input.strip_prefix(r"\w") {
            Ok((input, Self::Chars))
        } else if let Some(input) = input.strip_prefix(r"\1") {
            // NOTE: maybe improve this back reference parsing ...
            Ok((input, Self::BackReference(1)))
        } else if let Some(input) = input.strip_prefix(r"\2") {
            Ok((input, Self::BackReference(2)))
        } else if let Some(input) = input.strip_prefix(r"\3") {
            Ok((input, Self::BackReference(3)))
        } else if let Some(input) = input.strip_prefix(r"\4") {
            Ok((input, Self::BackReference(4)))
        } else if let Some(input) = input.strip_prefix(r"\5") {
            Ok((input, Self::BackReference(5)))
        } else if let Some(input) = input.strip_prefix('.') {
            Ok((input, Self::Wildcard))
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
            let mut delimiter_count = 1_isize;
            let mut parse_start = 0;
            let mut parse_end = 0;
            let mut sub_inputs = Vec::new();

            // Find end delimiter
            for (idx, c) in input.chars().enumerate() {
                match c {
                    '(' => delimiter_count += 1,
                    ')' => delimiter_count -= 1,
                    '|' if delimiter_count == 1 => {
                        sub_inputs.push(&input[parse_start..idx]);
                        parse_start = idx + 1;
                    }
                    _ => {}
                }

                if delimiter_count == 0 {
                    sub_inputs.push(&input[parse_start..idx]);
                    parse_end = idx;
                    break;
                }
            }

            if delimiter_count != 0 {
                return Err(GrepError::InvalidPattern);
            }

            let mut alternations = Vec::new();
            let id = alternation_counter.fetch_add(1, Ordering::Relaxed);

            for sub_sequence in sub_inputs {
                let sub_re = Regexp::parse(sub_sequence, alternation_counter)?;
                alternations.push(sub_re.patterns);
            }

            Ok((
                &input[parse_end + 1..],
                Self::Alternation { alternations, id },
            ))
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

fn match_here<'a>(
    patterns: &[Pattern],
    context: MatchContext<'a>,
) -> Option<(MatchResult, ReferenceTable<'a>)> {
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
        // Match back reference
        (_, Some((Pattern::BackReference(index), rem_patterns))) => {
            let reference = context.back_references.get(index)?;
            if !context.input_line[context.current_index..].starts_with(reference) {
                return None;
            }
            match_here(rem_patterns, context.nth_char(reference.len()))
        }
        // Match multiple chars
        (_, Some((Pattern::OneOrMore(pattern), rem_patterns))) => {
            // Match inner pattern with input patterns = match more
            match_here(&concat_pattern(pattern, patterns), context.clone()).or_else(||
                // Match inner pattern with remaining patterns = match one
                match_here(&concat_pattern(pattern, rem_patterns), context))
        }
        (_, Some((Pattern::ZeroOrOne(pattern), rem_patterns))) => {
            // Match one
            match_here(&concat_pattern(pattern, rem_patterns), context.clone()).or_else(||
                // Match zero
                match_here(rem_patterns, context))
        }
        (_, Some((Pattern::Alternation { alternations, id }, rem_patterns))) => {
            // For each possible alternation.
            for alt in alternations {
                // Check with line shorter than the whole input there is a negative char group in alternation.
                for end_index in (context.current_index..=context.input_line.len()).rev() {
                    // Create a new standalone context.
                    if let Some((alt_match, alt_ref_table)) = match_here(
                        alt,
                        MatchContext::new(context.current_index, &context.input_line[..end_index]),
                    ) {
                        // If alternation has match, merge everything output from result into current context.
                        let mut next_context = context
                            .nth_char(alt_match.1 - alt_match.0)
                            .with_back_reference(*id, alt_match);

                        next_context.back_references.extend(alt_ref_table.iter());

                        // Then try to match remaining patterns.
                        if let Some(((_, end_index), ref_table)) =
                            match_here(rem_patterns, next_context)
                        {
                            return Some(((context.start_index, end_index), ref_table));
                        }
                    }
                }
            }

            None
        }
        // Check end pattern
        (None, Some((Pattern::End, _rem_patterns))) => Some((
            (context.start_index, context.current_index),
            context.back_references,
        )),
        // If there is no more pattern
        (_, None) => Some((
            (context.start_index, context.current_index),
            context.back_references,
        )),
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

#[derive(Debug, Clone, Default)]
struct MatchContext<'a> {
    start_index: usize,
    current_index: usize,
    input_line: &'a str,
    back_references: ReferenceTable<'a>,
}

impl<'a> MatchContext<'a> {
    #[inline(always)]
    fn new(start_index: usize, input_line: &'a str) -> Self {
        Self {
            start_index,
            current_index: start_index,
            input_line,
            back_references: HashMap::new(),
        }
    }

    #[inline(always)]
    fn next_char(&self) -> Self {
        self.nth_char(1)
    }

    #[inline(always)]
    fn nth_char(&self, count: usize) -> Self {
        Self {
            start_index: self.start_index,
            current_index: self.current_index + count,
            input_line: self.input_line,
            back_references: self.back_references.clone(),
        }
    }

    #[inline(always)]
    fn with_back_reference(mut self, id: usize, pos: MatchResult) -> Self {
        self.back_references
            .insert(id, &self.input_line[pos.0..pos.1]);

        self
    }

    #[inline(always)]
    fn first_char(&self) -> Option<char> {
        self.input_line.chars().nth(self.current_index)
    }
}
