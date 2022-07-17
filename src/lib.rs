//! ZJSON is a study project for parsing with nom and combinatorial tokenization/parsing in general.

#![deny(missing_docs)]

use nom::{
    branch::alt,
    bytes::complete::{escaped_transform, tag},
    character::complete::{char, none_of, one_of},
    combinator::{opt, recognize},
    error::context,
    multi::{many0, many1},
    sequence::{delimited, preceded, terminated, tuple},
};

/// The type of token returned from the tokenizer.
#[derive(Debug, PartialEq)]
pub enum Token {
    /// A single/double quoted string delimited value.
    String(String),
    /// A floating point number.
    Number(f64),
}

/// Parse a single quoted string.
///
/// ## Example:
/// ```
/// # use zjson::*;
/// let (_, string) = parse_single_quoted_string("'hello'").unwrap();
/// assert_eq!(string, Token::String("hello".into()));
/// ```
pub fn parse_single_quoted_string(input: &str) -> nom::IResult<&str, Token> {
    context(
        "single_quoted_string",
        delimited(
            tag("'"),
            escaped_transform(none_of("\\'"), '\\', alt((tag("\\"), tag("'")))),
            tag("'"),
        ),
    )(input)
    .map(|(next_input, res)| (next_input, Token::String(res.into())))
}

/// Parse a double quoted string.
///
/// ## Example:
/// ```
/// # use zjson::*;
/// let (_, string) = parse_double_quoted_string("\"hello\"").unwrap();
/// assert_eq!(string, Token::String("hello".into()));
/// ```
pub fn parse_double_quoted_string(input: &str) -> nom::IResult<&str, Token> {
    context(
        "double_quoted_string",
        delimited(
            tag("\""),
            escaped_transform(none_of("\\\""), '\\', alt((tag("\\"), tag("\"")))),
            tag("\""),
        ),
    )(input)
    .map(|(next_input, res)| (next_input, Token::String(res)))
}

/// Parse a floating point number.
///
/// Borrowed from [Nom Recipes](https://docs.rs/nom/latest/nom/recipes/index.html#floating-point-numbers).
///
/// ## Example:
/// ```
/// # use zjson::*;
/// let (_, number) = parse_float("13.37").unwrap();
/// assert_eq!(number, Token::Number(13.37));;
/// ```
pub fn parse_float(input: &str) -> nom::IResult<&str, Token> {
    alt((
        // Case one: .42
        recognize(tuple((
            char('.'),
            parse_decimal,
            opt(tuple((one_of("eE"), opt(one_of("+-")), parse_decimal))),
        ))), // Case two: 42e42 and 42.42e42
        recognize(tuple((
            parse_decimal,
            opt(preceded(char('.'), parse_decimal)),
            one_of("eE"),
            opt(one_of("+-")),
            parse_decimal,
        ))), // Case three: 42. and 42.42
        recognize(tuple((parse_decimal, char('.'), opt(parse_decimal)))),
    ))(input)
    .map(|(next_input, res)| (next_input, Token::Number(res.parse::<f64>().unwrap())))
}

fn parse_decimal(input: &str) -> nom::IResult<&str, &str> {
    recognize(many1(terminated(one_of("0123456789"), many0(char('_')))))(input)
}

/// Parse a single or double quoted string.
///
/// ## Example:
/// ```
/// # use zjson::*;
/// let (_, string) = parse_string("\"hello\"").unwrap();
/// assert_eq!(string, Token::String("hello".into()));
///
/// let (_, string) = parse_string("'hello'").unwrap();
/// assert_eq!(string, Token::String("hello".into()));
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
            Token::String("hello".into())
        );
        assert!(parse_single_quoted_string("'yoted").is_err());
        assert!(parse_single_quoted_string("yoted'").is_err());
        assert!(parse_single_quoted_string("yoted").is_err());
    }

    #[test]
    fn test_parse_double_quoted_string() {
        assert_eq!(
            parse_double_quoted_string("\"hello\"").unwrap().1,
            Token::String("hello".into())
        );
        assert!(parse_double_quoted_string("\"yoted").is_err());
        assert!(parse_double_quoted_string("yoted\"").is_err());
        assert!(parse_double_quoted_string("yoted").is_err());
    }

    #[test]
    fn test_parse_string() {
        assert_eq!(
            parse_string("\"hello\"").unwrap().1,
            Token::String("hello".into())
        );
        assert!(parse_string("\"yoted'").is_err());
        assert!(parse_string("'yoted\"").is_err());

        assert_eq!(
            parse_string("'hello'").unwrap().1,
            Token::String("hello".into())
        );
        assert!(parse_string("'yoted\"").is_err());
        assert!(parse_string("\"yoted'").is_err());

        assert!(parse_string("yoted").is_err());
    }

    #[test]
    fn test_parse_decimal() {
        assert_eq!(parse_decimal("3000").unwrap().1, "3000");
        assert_eq!(parse_decimal("3_000_000").unwrap().1, "3_000_000");
        assert!(parse_decimal("_3_000_000").is_err());
    }

    #[test]
    fn test_parse_float() {
        assert_eq!(parse_float("13.37").unwrap().1, Token::Number(13.37));
        assert_eq!(parse_float(".37").unwrap().1, Token::Number(0.37));
        assert_eq!(parse_float("10e4").unwrap().1, Token::Number(10.0e4));
    }
}
