/*
 *  Copyright (C) 2021  Wafelack
 * 
 *  This file is part of GVLC.
 *
 *  GVLC is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  GVLC is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with GVLC.  If not, see <https://www.gnu.org/licenses/>.
 */
use crate::{
    lexer::{Lexer, TType, Token},
    parser::{Expr, ExprT, Parser},
    compiler::{Compiler},
    VLispResult,
};

mod compiler {
    use super::*;

    #[test]
    fn set() -> VLispResult<()> {
        let tokens = Lexer::new("(set bar)(set foo 'toggle)(set foo 'off)(set bar 'vi)(set moo 'vim)(set foobar 'reset)").proc_tokens()?;
        let expressions = Parser::new(tokens).parse()?;
        let output = Compiler::new(expressions).compile()?;
        assert_eq!(output.as_str(), "set bar\nset invfoo\nset nofoo\nset bar&vi\nset moo&vim\nset foobar&\n");
        Ok(())

    }

    #[test]
    fn operators() -> VLispResult<()> {
        let tokens = Lexer::new("(!= 3 4)(and (== 3 4) (== 3 5))").proc_tokens()?;
        let expressions = Parser::new(tokens).parse()?;
        let output = Compiler::new(expressions).compile()?;
        assert_eq!(output.as_str(), "(3 != 4)\n((3 == 4) && (3 == 5))\n");
        Ok(())
    }


    #[test]
    fn map() -> VLispResult<()> {
        let tokens = Lexer::new(r#"(map "<leader>foo" 5 'normal 'recursive 'buffer)"#).proc_tokens()?;
        let expressions = Parser::new(tokens).parse()?;
        let output = Compiler::new(expressions).compile()?;
        assert_eq!(output.as_str(), "nmap <buffer> <leader>foo 5\n");
        Ok(())
    }

    #[test]
    fn r#let() -> VLispResult<()> {
        let tokens = Lexer::new(r#"(let foo "bar" 'script)"#).proc_tokens()?;
        let expressions = Parser::new(tokens).parse()?;
        let output = Compiler::new(expressions).compile()?;
        assert_eq!(output.as_str(), "let s:foo = \"bar\"\n");
        Ok(())
    }
    #[test]
    fn get() -> VLispResult<()> {
        let tokens = Lexer::new("(get)(get foo)(get 'all)(get 'termcap 'newline)").proc_tokens()?;
        let expressions = Parser::new(tokens).parse()?;
        let output = Compiler::new(expressions).compile()?;
        assert_eq!(output.as_str(), "set\nset foo?\nset all\nset! termcap\n");
        Ok(())
    }
    #[test]
    fn defun() -> VLispResult<()> {
        let tokens = Lexer::new("(defun 'script (foo bar))(defun 'no-overwrite (bar moo foo))(defun 'abort (moo) (map \"foo\" 98))").proc_tokens()?;
        let expressions = Parser::new(tokens).parse()?;
        let output = Compiler::new(expressions).compile()?;
        assert_eq!(output.as_str(), "function! s:foo(bar)\nendfunction\nfunction bar(moo, foo)\nendfunction\nfunction! moo() abort\nnoremap  foo 98\nendfunction\n");
        Ok(())
    }

    #[test]
    fn cond() -> VLispResult<()> {
        let tokens = Lexer::new("(cond [foo 5] [bar 6] [else 3])").proc_tokens()?;
        let expressions = Parser::new(tokens).parse()?;
        let output = Compiler::new(expressions).compile()?;
        assert_eq!(output.as_str(), "if foo\n5\nelseif bar\n6\nelse \n3\nendif\n");
        Ok(())
    }


    #[test]
    fn arrays() -> VLispResult<()> {
        let tokens = Lexer::new("[4 5 6 4]").proc_tokens()?;
        let expressions = Parser::new(tokens).parse()?;
        let output = Compiler::new(expressions).compile()?;
        assert_eq!(output.as_str(), "[4,5,6,4]\n");
        Ok(())
    }

}

mod parser {
    use super::*;

    fn types_from_expresssions(expressions: Vec<Expr>) -> Vec<ExprT> {
        expressions
            .iter()
            .map(|t| t.exprt.clone())
            .collect::<Vec<_>>()
    }

    #[test]
    fn string() -> VLispResult<()> {
        let tokens = Lexer::new(r#""Hello, World !""#).proc_tokens()?;
        let expressions = types_from_expresssions(Parser::new(tokens).parse()?);
        assert_eq!(
            expressions,
            vec![ExprT::String("Hello, World !".to_string())]
            );

        Ok(())
    }

    #[test]
    fn number() -> VLispResult<()> {
        let tokens = Lexer::new("55").proc_tokens()?;
        let expressions = types_from_expresssions(Parser::new(tokens).parse()?);
        assert_eq!(expressions, vec![ExprT::Number(55)]);

        Ok(())
    }

    #[test]
    fn float() -> VLispResult<()> {
        let tokens = Lexer::new("3.1415").proc_tokens()?;
        let expressions = types_from_expresssions(Parser::new(tokens).parse()?);
        assert_eq!(expressions, vec![ExprT::Float(3.1415)]);

        Ok(())
    }
    #[test]
    fn symbol() -> VLispResult<()> {
        let tokens = Lexer::new("'recursive").proc_tokens()?;
        let expressions = types_from_expresssions(Parser::new(tokens).parse()?);
        assert_eq!(expressions, vec![ExprT::Symbol("recursive".to_string())]);

        Ok(())
    }

    #[test]
    fn array() -> VLispResult<()> {
        let tokens = Lexer::new(r#"[]"#).proc_tokens()?;
        let expressions = types_from_expresssions(Parser::new(tokens).parse()?);
        assert_eq!(
            expressions,
            vec![ExprT::Array(
                vec![]
                )]
            );

        Ok(())

    }

    #[test]
    fn call() -> VLispResult<()> {
        let tokens = Lexer::new("(call foo)").proc_tokens()?;
        let expressions = types_from_expresssions(Parser::new(tokens).parse()?);
        assert_eq!(
            expressions,
            vec![ExprT::Call(
                "call".to_string(),
                vec!(Expr::new(ExprT::Identifier("foo".to_string()), 1, 9))
                )]
            );

        Ok(())
    }
}

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
    #[test]
    fn brackets() -> VLispResult<()> {
        let ttypes = types_from_tokens(Lexer::new("[]").proc_tokens()?);
        assert_eq!(ttypes, vec![TType::LBracket, TType::RBracket]);
        Ok(())
    }

    #[test]
    fn quote() -> VLispResult<()> {
        let ttypes = types_from_tokens(Lexer::new("'").proc_tokens()?);
        assert_eq!(ttypes, vec![TType::Quote]);
        Ok(())
    }
}
