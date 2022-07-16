//! ZJSON is a study project for parsing with nom and combinatorial tokenization/parsing in general.

#![deny(missing_docs)]

use nom::{bytes::complete::tag, error::context, sequence::delimited};

/// The type of token returned from the tokenizer.
#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Token<'a> {
    /// A single/double quoted string delimited value.
    String(&'a str),
}

/// Parse a single quoted string.
/// ```
/// # use zjson::*;
/// let (_, string) = parse_single_quoted_string("'hello'").unwrap();
/// assert_eq!(string, Token::String("hello"));
/// ```
pub fn parse_single_quoted_string(input: &str) -> nom::IResult<&str, Token> {
    context("string", delimited(tag("'"), tag("hello"), tag("'")))(input)
        .map(|(next_input, res)| (next_input, Token::String(res.into())))
}

/// Parse a double quoted string.
/// ```
/// # use zjson::*;
/// let (_, string) = parse_double_quoted_string("\"hello\"").unwrap();
/// assert_eq!(string, Token::String("hello"));
/// ```
pub fn parse_double_quoted_string(input: &str) -> nom::IResult<&str, Token> {
    context("string", delimited(tag("\""), tag("hello"), tag("\"")))(input)
        .map(|(next_input, res)| (next_input, Token::String(res.into())))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_expression() {
        assert_eq!(
            parse_single_quoted_string("'hello'").unwrap().1,
            Token::String("hello")
        );
        assert!(parse_single_quoted_string("'yoted").is_err());
        assert!(parse_single_quoted_string("yoted'").is_err());
        assert!(parse_single_quoted_string("yoted").is_err());
    }
}
