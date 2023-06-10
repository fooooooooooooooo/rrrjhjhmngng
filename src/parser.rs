use std::vec::IntoIter;

use chumsky::input::Stream;
use chumsky::prelude::*;
use chumsky::{input::SpannedInput, span::SimpleSpan};
use logos::Logos;

use crate::lexer::Token;

#[derive(Debug, PartialEq)]
pub struct Format<'a>(pub Option<&'a str>, pub Option<(f32, f32, f32)>);

type ParserError<'a, T> = extra::Err<Rich<'a, T>>;
type ParserInput<'a> =
    SpannedInput<Token<'a>, SimpleSpan, Stream<IntoIter<(Token<'a>, SimpleSpan)>>>;

fn number_parser<'i>() -> impl Parser<'i, ParserInput<'i>, f32, ParserError<'i, Token<'i>>> + Clone
{
    let number = select! { Token::Number(i) => i };

    number.from_str::<f32>().unwrapped()
}

fn define_parser<'i>() -> impl Parser<'i, ParserInput<'i>, (), ParserError<'i, Token<'i>>> + Clone {
    select! { Token::Define => () }
}

fn identifier_parser<'i>(
) -> impl Parser<'i, ParserInput<'i>, &'i str, ParserError<'i, Token<'i>>> + Clone {
    select! { Token::Identifier(id) => id }
}

fn format_parser<'i>(
) -> impl Parser<'i, ParserInput<'i>, Format<'i>, ParserError<'i, Token<'i>>> + Clone {
    let format_name = identifier_parser();

    let format_values = number_parser()
        .repeated()
        .at_least(3)
        .at_most(3)
        .collect_exactly::<[f32; 3]>()
        .map(|v| (v[0], v[1], v[2]))
        .delimited_by(just(Token::LParen), just(Token::RParen));

    just(Token::Dot)
        .ignore_then(format_name)
        .or_not()
        .then(format_values.or_not())
        .map(|(name, values)| Format(name, values))
}

pub type Color<'a> = (&'a str, &'a str, Format<'a>);
pub type ParseError<'a> = Rich<'a, Token<'a>>;
pub type Errors<'a> = (Vec<ParseError<'a>>, Vec<ParseError<'a>>);

pub fn parse(input: &str) -> (Option<Vec<Color>>, Errors) {
    let mut tokenizer_errors = Vec::new();

    let tokens = Token::lexer(input)
        .spanned()
        .filter_map(|(token, range)| {
            let span = SimpleSpan::from(range.clone());

            match token {
                Err(_) => {
                    tokenizer_errors.push(ParseError::custom(
                        span,
                        format!("invalid token '{}'", &input[range]),
                    ));

                    None
                }
                Ok(token) => Some((token, span)),
            }
        })
        .collect::<Vec<_>>();

    let length = tokens.len();
    let tokens_stream: ParserInput<'_> = Stream::from_iter(tokens).spanned((length..length).into());

    let (colors, parse_errors) = parser().parse(tokens_stream).into_output_errors();

    (colors, (tokenizer_errors, parse_errors))
}

fn parser<'i>() -> impl Parser<'i, ParserInput<'i>, Vec<Color<'i>>, ParserError<'i, Token<'i>>> {
    let name = identifier_parser();
    let key = identifier_parser();

    let values = key.then(format_parser()).delimited_by(
        just(Token::Asperand).then(just(Token::LBracket)),
        just(Token::RBracket),
    );

    let color = name
        .then(values)
        .map(|(name, (key, format))| (name, key, format))
        .then_ignore(just(Token::Semi));

    define_parser()
        .or_not()
        .ignore_then(color)
        .repeated()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stream_token_vec(
        tokens: Vec<Token>,
    ) -> SpannedInput<Token, SimpleSpan, Stream<IntoIter<(Token, SimpleSpan)>>> {
        let tokens = tokens
            .into_iter()
            .map(|t| (t, SimpleSpan::from(0usize..0)))
            .collect::<Vec<_>>();

        let length = tokens.len();
        Stream::from_iter(tokens).spanned((length..length).into())
    }

    #[test]
    fn number_parser_works() {
        let tests = vec![
            (Token::Number("10"), 10f32),
            (Token::Number("49.123"), 49.123f32),
            (Token::Number("-124"), -124f32),
            (Token::Number("-618.4368"), -618.4368f32),
        ];

        for (token, expected) in tests {
            let res = number_parser()
                .parse(stream_token_vec(vec![token]))
                .into_result();

            assert!(res.is_ok());

            let val = res.unwrap();

            assert_eq!(expected, val);
        }
    }

    #[test]
    fn ident_parser_works() {
        let tests = vec![
            (Token::Identifier("swag"), "swag"),
            (Token::Identifier("sw_ag"), "sw_ag"),
            (Token::Identifier("swag23"), "swag23"),
        ];

        for (token, expected) in tests {
            let res = identifier_parser()
                .parse(stream_token_vec(vec![token]))
                .into_result();

            assert!(res.is_ok());

            let val = res.unwrap();

            assert_eq!(expected, val);
        }
    }
}
