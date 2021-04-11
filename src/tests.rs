use crate::{VLispResult, lexer::{Lexer, TType, Token}};

#[cfg(test)]
mod test {
    use super::*;

    mod lexer {
        use super::*;

        fn types_from_tokens(tokens: Vec<Token>) -> Vec<TType> {
            tokens.iter().map(|t| t.ttype.clone()).collect::<Vec<_>>()
        }

        #[test]
        fn parentheses() -> VLispResult<()> {
            let ttypes = types_from_tokens(Lexer::new("()").proc_tokens()?);
            assert_eq!(ttypes, vec![TType::LParen, TType::RParen]);
            Ok(())
        }

        #[test]
        fn number() -> VLispResult<()> {
            let ttypes = types_from_tokens(Lexer::new("42").proc_tokens()?);
            assert_eq!(ttypes, vec![TType::Number(42)]);
            Ok(())
        }

        #[test]
        fn float() -> VLispResult<()> {
            let ttypes = types_from_tokens(Lexer::new("3.1415").proc_tokens()?);
            assert_eq!(ttypes, vec![TType::Float(3.1415)]);
            Ok(())
        }

        #[test]
        fn string() -> VLispResult<()> {
            let ttypes = types_from_tokens(Lexer::new(r#""Hello, World !""#).proc_tokens()?);
            assert_eq!(ttypes, vec![TType::String("Hello, World !".to_string())]);
            Ok(())
        }

        #[test]
        fn identifier() -> VLispResult<()> {
            let ttypes = types_from_tokens(Lexer::new("define").proc_tokens()?);
            assert_eq!(ttypes, vec![TType::Ident("define".to_string())]);
            Ok(())
        }
    }
}
