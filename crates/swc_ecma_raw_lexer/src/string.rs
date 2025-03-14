use logos::{Lexer, Logos};

use crate::{token::TokenState, LogosError, TokenType, TokenValue};

pub fn consume_str_single_quote(lex: &mut Lexer<TokenType>) -> Result<(), LogosError> {
    consume_str(lex, StrContent::SingleQuote)
}

pub fn consume_str_double_quote(lex: &mut Lexer<TokenType>) -> Result<(), LogosError> {
    consume_str(lex, StrContent::DoubleQuote)
}

fn consume_str(lex: &mut Lexer<TokenType>, stop_token: StrContent) -> Result<(), LogosError> {
    let mut str_lexer = lex.clone().morph::<StrContent>();

    while let Some(Ok(token)) = str_lexer.next() {
        if token == stop_token {
            str_lexer.extras.value = TokenValue::Str { value: (), raw: () };

            *lex = str_lexer.morph();

            return Ok(());
        }
    }

    Err(LogosError::UnterminatedStr)
}

#[derive(Logos, Debug, Clone, Copy, PartialEq, Eq)]
#[logos(error = LogosError, extras = TokenState)]
enum StrContent {
    #[regex(r#"\\["'\\bfnrtv]"#, priority = 100)]
    #[regex(r#"\\0[0-7]*"#, priority = 100)]
    #[regex(r#"\\x[0-9a-fA-F]{2}"#, priority = 100)]
    #[regex(r#"\\u[0-9a-fA-F]{4}"#, priority = 100)]
    #[regex(r#"\\[^'"\\]+"#)]
    Escape,

    #[regex(r#"[^'"\\]+"#)]
    Normal,

    #[regex(r#"'"#)]
    SingleQuote,

    #[regex(r#"""#)]
    DoubleQuote,
}

#[cfg(test)]
mod tests {
    use logos::Lexer;
    use pretty_assertions::assert_eq;

    use super::StrContent;

    fn assert_str(text: &str, expected: &[StrContent]) {
        dbg!(text);
        dbg!(expected);

        let actual = Lexer::<StrContent>::new(text)
            .map(|v| v.unwrap())
            .collect::<Vec<_>>();

        let mut lexer = Lexer::<StrContent>::new(text);

        while let Some(Ok(token)) = lexer.next() {
            dbg!(&token);
            dbg!(lexer.slice());
        }

        // Actual contains the last quote

        assert_eq!(expected.len() + 1, actual.len());
        assert_eq!(expected, &actual[..expected.len()]);

        assert!(matches!(
            actual.last(),
            Some(StrContent::SingleQuote | StrContent::DoubleQuote)
        ));
    }

    #[test]
    fn test_newline() {
        assert_str(
            "hello\\nworld'",
            &[StrContent::Normal, StrContent::Escape, StrContent::Normal],
        );
    }

    #[test]
    fn test_escape() {
        assert_str(r#"\''"#, &[StrContent::Escape]);
    }

    #[test]
    fn test_escape_escape() {
        assert_str(r#"\\'"#, &[StrContent::Escape]);
    }
}
