/*
 *  Copyright (C) 2021  Wafelack
 *
 *  This file is part of Vinal.
 *
 *  Vinal is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  Vinal is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with Vinal.  If not, see <https://www.gnu.org/licenses/>.
 */
use crate::{
    compiler::Compiler,
    lexer::{Lexer, TType, Token},
    parser::{Expr, ExprT, Parser},
    VinalResult,
};

mod compiler {
    use super::*;

    #[test]
    fn set() -> VinalResult<()> {
        let tokens = Lexer::new("(set bar)(set foo 'toggle)(set foo 'off)(set bar 'vi)(set moo 'vim)(set foobar 'reset)").proc_tokens()?;
        let expressions = Parser::new(tokens).parse()?;
        let output = Compiler::new(expressions).compile()?;
        assert_eq!(
            output.as_str(),
            "set bar\nset invfoo\nset nofoo\nset bar&vi\nset moo&vim\nset foobar&\n"
        );
        Ok(())
    }

    #[test]
    fn operators() -> VinalResult<()> {
        let tokens = Lexer::new("(!= 3 4)(and (== 3 4) (== 3 5))").proc_tokens()?;
        let expressions = Parser::new(tokens).parse()?;
        let output = Compiler::new(expressions).compile()?;
        assert_eq!(output.as_str(), "(3 != 4)\n((3 == 4) && (3 == 5))\n");
        Ok(())
    }

    #[test]
    fn map() -> VinalResult<()> {
        let tokens =
            Lexer::new(r#"(map "<leader>foo" 5 'normal 'recursive 'buffer)"#).proc_tokens()?;
        let expressions = Parser::new(tokens).parse()?;
        let output = Compiler::new(expressions).compile()?;
        assert_eq!(output.as_str(), "nmap <buffer> <leader>foo 5\n");
        Ok(())
    }

    #[test]
    fn r#let() -> VinalResult<()> {
        let tokens = Lexer::new(r#"(let foo "bar" 'script)"#).proc_tokens()?;
        let expressions = Parser::new(tokens).parse()?;
        let output = Compiler::new(expressions).compile()?;
        assert_eq!(output.as_str(), "let s:foo = \"bar\"\n");
        Ok(())
    }

    #[test]
    fn call() -> VinalResult<()> {
        let tokens = Lexer::new("(call foo 5 6)").proc_tokens()?;
        let expressions = Parser::new(tokens).parse()?;
        let output = Compiler::new(expressions).compile()?;
        assert_eq!(output.as_str(), "call foo(5,6)\n");
        Ok(())
    }

    #[test]
    fn get() -> VinalResult<()> {
        let tokens = Lexer::new("(get)(get foo)(get 'all)(get 'termcap 'newline)").proc_tokens()?;
        let expressions = Parser::new(tokens).parse()?;
        let output = Compiler::new(expressions).compile()?;
        assert_eq!(output.as_str(), "set\nset foo?\nset all\nset! termcap\n");
        Ok(())
    }
    #[test]
    fn defun() -> VinalResult<()> {
        let tokens = Lexer::new("(defun 'script (foo bar))(defun 'no-overwrite (bar moo foo))(defun 'abort (moo) (map \"foo\" 98))").proc_tokens()?;
        let expressions = Parser::new(tokens).parse()?;
        let output = Compiler::new(expressions).compile()?;
        assert_eq!(output.as_str(), "function! s:foo(bar)\nendfunction\nfunction bar(moo, foo)\nendfunction\nfunction! moo() abort\nnoremap  foo 98\nendfunction\n");
        Ok(())
    }

    #[test]
    fn cond() -> VinalResult<()> {
        let tokens = Lexer::new("(cond [foo 5] [bar 6] [else 3])").proc_tokens()?;
        let expressions = Parser::new(tokens).parse()?;
        let output = Compiler::new(expressions).compile()?;
        assert_eq!(
            output.as_str(),
            "if foo\n5\nelseif bar\n6\nelse \n3\nendif\n"
        );
        Ok(())
    }

    #[test]
    fn arrays() -> VinalResult<()> {
        let tokens = Lexer::new("[4 5 6 4]").proc_tokens()?;
        let expressions = Parser::new(tokens).parse()?;
        let output = Compiler::new(expressions).compile()?;
        assert_eq!(output.as_str(), "[4,5,6,4]\n");
        Ok(())
    }

    #[test]
    fn source() -> VinalResult<()> {
        let tokens = Lexer::new(r#"(source "foo.vim")(source $MYVIMRC 'normal)"#).proc_tokens()?;
        let expressions = Parser::new(tokens).parse()?;
        let output = Compiler::new(expressions).compile()?;
        assert_eq!(output.as_str(), "source foo.vim\nsource! $MYVIMRC\n");
        Ok(())
    }

    #[test]
    fn edit() -> VinalResult<()> {
        let tokens =
            Lexer::new(r#"(edit)(edit 'discard)(edit "foo.toml")(edit 'discard $MYVIMRC)"#)
                .proc_tokens()?;
        let expressions = Parser::new(tokens).parse()?;
        let output = Compiler::new(expressions).compile()?;
        assert_eq!(
            output.as_str(),
            "edit\nedit!\nedit foo.toml\nedit! $MYVIMRC\n"
        );
        Ok(())
    }

    #[test]
    fn marks() -> VinalResult<()> {
        let tokens = Lexer::new("(mark m)(goto m)").proc_tokens()?;
        let expressions = Parser::new(tokens).parse()?;
        let output = Compiler::new(expressions).compile()?;
        assert_eq!(output.as_str(), "mm\n`m\n");
        Ok(())
    }

    #[test]
    fn colorscheme() -> VinalResult<()> {
        let tokens = Lexer::new(r#"(colorscheme)(colorscheme "horizon")"#).proc_tokens()?;
        let expressions = Parser::new(tokens).parse()?;
        let output = Compiler::new(expressions).compile()?;
        assert_eq!(output.as_str(), "colorscheme\ncolorscheme horizon\n");
        Ok(())
    }

    #[test]
    fn any() -> VinalResult<()> {
        let tokens = Lexer::new(r#"(Plug "foobar/moo" 'command)(moo "foo" 42)"#).proc_tokens()?;
        let expressions = Parser::new(tokens).parse()?;
        let output = Compiler::new(expressions).compile()?;
        assert_eq!(output.as_str(), "Plug \"foobar/moo\"\nmoo(\"foo\", 42)\n");
        Ok(())
    }

    #[test]
    fn raw() -> VinalResult<()> {
        let tokens =
            Lexer::new(r#"(raw "nnoremap <buffer> <leader>i mmgg=G`m" "echom \"foobar\"")"#)
                .proc_tokens()?;
        let expressions = Parser::new(tokens).parse()?;
        let output = Compiler::new(expressions).compile()?;
        assert_eq!(
            output.as_str(),
            "nnoremap <buffer> <leader>i mmgg=G`m\nechom \"foobar\"\n\n"
        );
        Ok(())
    }

    #[test]
    fn gotab() -> VinalResult<()> {
        let tokens = Lexer::new("(gotab)(gotab 2)").proc_tokens()?;
        let expressions = Parser::new(tokens).parse()?;
        let output = Compiler::new(expressions).compile()?;
        assert_eq!(output.as_str(), "gt\n2gt\n");
        Ok(())
    }

    #[test]
    fn dict() -> VinalResult<()> {
        let tokens = Lexer::new(
            r#"(dict 
            "foo" "bar"
            5 "moo")"#,
        )
        .proc_tokens()?;
        let expressions = Parser::new(tokens).parse()?;
        let output = Compiler::new(expressions).compile()?;
        assert_eq!(output.as_str(), "{ \"foo\":\"bar\", 5:\"moo\" }\n");
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
    fn string() -> VinalResult<()> {
        let tokens = Lexer::new(r#""Hello, World !""#).proc_tokens()?;
        let expressions = types_from_expresssions(Parser::new(tokens).parse()?);
        assert_eq!(
            expressions,
            vec![ExprT::String("Hello, World !".to_string())]
        );

        Ok(())
    }

    #[test]
    fn number() -> VinalResult<()> {
        let tokens = Lexer::new("55").proc_tokens()?;
        let expressions = types_from_expresssions(Parser::new(tokens).parse()?);
        assert_eq!(expressions, vec![ExprT::Number(55)]);

        Ok(())
    }

    #[test]
    fn float() -> VinalResult<()> {
        let tokens = Lexer::new("3.1415").proc_tokens()?;
        let expressions = types_from_expresssions(Parser::new(tokens).parse()?);
        assert_eq!(expressions, vec![ExprT::Float(3.1415)]);

        Ok(())
    }
    #[test]
    fn symbol() -> VinalResult<()> {
        let tokens = Lexer::new("'recursive").proc_tokens()?;
        let expressions = types_from_expresssions(Parser::new(tokens).parse()?);
        assert_eq!(expressions, vec![ExprT::Symbol("recursive".to_string())]);

        Ok(())
    }

    #[test]
    fn array() -> VinalResult<()> {
        let tokens = Lexer::new(r#"[]"#).proc_tokens()?;
        let expressions = types_from_expresssions(Parser::new(tokens).parse()?);
        assert_eq!(expressions, vec![ExprT::Array(vec![])]);

        Ok(())
    }

    #[test]
    fn call() -> VinalResult<()> {
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
    fn parentheses() -> VinalResult<()> {
        let ttypes = types_from_tokens(Lexer::new("()").proc_tokens()?);
        assert_eq!(ttypes, vec![TType::LParen, TType::RParen]);
        Ok(())
    }

    #[test]
    fn number() -> VinalResult<()> {
        let ttypes = types_from_tokens(Lexer::new("42").proc_tokens()?);
        assert_eq!(ttypes, vec![TType::Number(42)]);
        Ok(())
    }

    #[test]
    fn float() -> VinalResult<()> {
        let ttypes = types_from_tokens(Lexer::new("3.1415").proc_tokens()?);
        assert_eq!(ttypes, vec![TType::Float(3.1415)]);
        Ok(())
    }

    #[test]
    fn string() -> VinalResult<()> {
        let ttypes = types_from_tokens(Lexer::new(r#""Hello, World !""#).proc_tokens()?);
        assert_eq!(ttypes, vec![TType::String("Hello, World !".to_string())]);
        Ok(())
    }

    #[test]
    fn identifier() -> VinalResult<()> {
        let ttypes = types_from_tokens(Lexer::new("define").proc_tokens()?);
        assert_eq!(ttypes, vec![TType::Ident("define".to_string())]);
        Ok(())
    }
    #[test]
    fn brackets() -> VinalResult<()> {
        let ttypes = types_from_tokens(Lexer::new("[]").proc_tokens()?);
        assert_eq!(ttypes, vec![TType::LBracket, TType::RBracket]);
        Ok(())
    }

    #[test]
    fn quote() -> VinalResult<()> {
        let ttypes = types_from_tokens(Lexer::new("'").proc_tokens()?);
        assert_eq!(ttypes, vec![TType::Quote]);
        Ok(())
    }
}
