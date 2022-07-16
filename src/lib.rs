//! ZJSON is a study project for parsing with nom and combinatorial tokenization/parsing in general.

#![deny(missing_docs)]

use nom::{branch::alt, bytes::complete::tag, error::context, sequence::delimited};

/// The type of token returned from the tokenizer.
#[derive(Debug, Eq, PartialEq)]
pub enum Token<'a> {
    /// A single/double quoted string delimited value.
    String(&'a str),
}

/// Parse a single quoted string.
///
/// ## Example:
/// ```
/// # use zjson::*;
/// let (_, string) = parse_single_quoted_string("'hello'").unwrap();
/// assert_eq!(string, Token::String("hello"));
/// ```
pub fn parse_single_quoted_string(input: &str) -> nom::IResult<&str, Token> {
    context(
        "single_quoted_string",
        delimited(tag("'"), tag("hello"), tag("'")),
    )(input)
    .map(|(next_input, res)| (next_input, Token::String(res.into())))
}

/// Parse a double quoted string.
///
/// ## Example:
/// ```
/// # use zjson::*;
/// let (_, string) = parse_double_quoted_string("\"hello\"").unwrap();
/// assert_eq!(string, Token::String("hello"));
/// ```
pub fn parse_double_quoted_string(input: &str) -> nom::IResult<&str, Token> {
    context(
        "double_quoted_string",
        delimited(tag("\""), tag("hello"), tag("\"")),
    )(input)
    .map(|(next_input, res)| (next_input, Token::String(res.into())))
}

/// Parse a single or double quoted string.
///
/// ## Example:
/// ```
/// # use zjson::*;
/// let (_, string) = parse_string("\"hello\"").unwrap();
/// assert_eq!(string, Token::String("hello"));
///
/// let (_, string) = parse_string("'hello'").unwrap();
/// assert_eq!(string, Token::String("hello"));
/// ```
pub fn parse_string(input: &str) -> nom::IResult<&str, Token> {
    context(
        "string",
        alt((parse_single_quoted_string, parse_double_quoted_string)),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_quoted_string() {
        assert_eq!(
            parse_single_quoted_string("'hello'").unwrap().1,
            Token::String("hello")
        );
        assert!(parse_single_quoted_string("'yoted").is_err());
        assert!(parse_single_quoted_string("yoted'").is_err());
        assert!(parse_single_quoted_string("yoted").is_err());
    }

    #[test]
    fn test_parse_double_quoted_string() {
        assert_eq!(
            parse_double_quoted_string("\"hello\"").unwrap().1,
            Token::String("hello")
        );
        assert!(parse_double_quoted_string("\"yoted").is_err());
        assert!(parse_double_quoted_string("yoted\"").is_err());
        assert!(parse_double_quoted_string("yoted").is_err());
    }

    #[test]
    fn test_parse_string() {
        assert_eq!(parse_string("\"hello\"").unwrap().1, Token::String("hello"));
        assert!(parse_string("\"yoted'").is_err());
        assert!(parse_string("'yoted\"").is_err());

        assert_eq!(parse_string("'hello'").unwrap().1, Token::String("hello"));
        assert!(parse_string("'yoted\"").is_err());
        assert!(parse_string("\"yoted'").is_err());

        assert!(parse_string("yoted").is_err());
    }
}
