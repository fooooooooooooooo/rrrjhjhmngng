use std::fmt::Display;

use logos::Logos;

#[derive(Logos, Clone, Debug, PartialEq)]
#[logos(skip r"[\s\f\r]+")]
pub enum Token<'input> {
    #[token("@define-color")]
    Define,
    #[regex(r#"[\p{L}_][\p{L}\d_]*"#)]
    Identifier(&'input str),
    #[regex(r#"-?\d+(?:\.\d+)?"#)]
    Number(&'input str),
    #[token(".")]
    Dot,
    #[token("@")]
    Asperand,
    #[token("{")]
    LBracket,
    #[token("}")]
    RBracket,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token(";")]
    Semi,
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Token::Define => write!(f, "@define-color"),
            Token::Identifier(id) => write!(f, "{id}"),
            Token::Number(num) => write!(f, "{num}"),
            Token::Dot => write!(f, "."),
            Token::Asperand => write!(f, "@"),
            Token::LBracket => write!(f, "{{"),
            Token::RBracket => write!(f, "}}"),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::Semi => write!(f, ";"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_lexer_works() {
        let input = "1 2.0 3.1 4.234 5 123456789 9223372036854775807";
        let lexer = Token::lexer(input);
        let tokens: Vec<_> = lexer.into_iter().collect();

        assert_eq!(
            tokens,
            vec![
                Ok(Token::Number("1")),
                Ok(Token::Number("2.0")),
                Ok(Token::Number("3.1")),
                Ok(Token::Number("4.234")),
                Ok(Token::Number("5")),
                Ok(Token::Number("123456789")),
                Ok(Token::Number("9223372036854775807")),
            ]
        )
    }

    #[test]
    fn id_lexer_works() {
        let input = "abc123 _abc123 Abc_123 abc_123 _ aBC_123_def ABC_123_DEF a1_b2_c3";
        let lexer = Token::lexer(input);
        let tokens: Vec<_> = lexer.into_iter().collect();

        assert_eq!(
            tokens,
            vec![
                Ok(Token::Identifier("abc123")),
                Ok(Token::Identifier("_abc123")),
                Ok(Token::Identifier("Abc_123")),
                Ok(Token::Identifier("abc_123")),
                Ok(Token::Identifier("_")),
                Ok(Token::Identifier("aBC_123_def")),
                Ok(Token::Identifier("ABC_123_DEF")),
                Ok(Token::Identifier("a1_b2_c3")),
            ]
        )
    }
}
