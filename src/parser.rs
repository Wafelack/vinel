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
    lexer::{TType, Token},
    VinalResult,
};
use std::mem::discriminant;

#[derive(Clone, Debug, PartialEq)]
pub enum ExprT {
    String(String),
    Number(i32),
    Float(f32),
    Symbol(String),
    Call(String, Vec<Expr>),
    Array(Vec<Expr>),
    Identifier(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
    pub exprt: ExprT,
    pub line: usize,
    pub column: usize,
}
impl Expr {
    pub fn new(exprt: ExprT, line: usize, column: usize) -> Self {
        Self {
            exprt,
            line,
            column,
        }
    }
    pub fn get_type(&self) -> String {
        match self.exprt {
            ExprT::String(_) => "String",
            ExprT::Number(_) => "Number",
            ExprT::Float(_) => "Float",
            ExprT::Symbol(_) => "Symbol",
            ExprT::Call(_, _) => "Function Call",
            ExprT::Identifier(_) => "Variable",
            ExprT::Array(_) => "Array",
        }
        .to_string()
    }
}

pub struct Parser {
    input: Vec<Token>,
    output: Vec<Expr>,
    current: usize,
}
impl Parser {
    pub fn new(input: Vec<Token>) -> Self {
        Self {
            input,
            output: vec![],
            current: 0,
        }
    }
    fn is_at_end(&self) -> bool {
        self.current >= self.input.len()
    }
    fn advance(&mut self, expected: TType) -> Result<Token, String> {
        let popped = self.pop()?;

        if discriminant(&popped.ttype) != discriminant(&expected) {
            Err(format!(
                "{}:{} | Expected token of type {}, found one of type {}.",
                popped.line,
                popped.column,
                expected.get_type(),
                popped.ttype.get_type()
            ))
        } else {
            Ok(popped)
        }
    }
    fn pop(&mut self) -> Result<Token, String> {
        if self.is_at_end() {
            let previous = &self.input[self.current];
            Err(format!(
                "{}:{} | Unexpected EOF while parsing expression.",
                previous.line, previous.column
            ))
        } else {
            if self.input.len() != 1 {
                self.current += 1;
            }
            Ok(self.input[self.current - if self.input.len() == 1 { 0 } else { 1 }].clone())
        }
    }
    fn peek(&self) -> Option<Token> {
        self.input
            .iter()
            .nth(self.current)
            .and_then(|t| Some(t.clone()))
    }
    pub fn parse(&mut self) -> VinalResult<Vec<Expr>> {
        let mut errors = vec![];
        while !self.is_at_end() {
            let to_psh = match self.parse_expr() {
                Ok(e) => e,
                Err(e) => {
                    errors.push(e);
                    continue;
                }
            };

            self.output.push(to_psh);

            if self.input.len() == 1 {
                break;
            }
        }

        if errors.is_empty() {
            Ok(self.output.clone())
        } else {
            Err(errors)
        }
    }
    fn parse_expr(&mut self) -> Result<Expr, String> {
        let token = self.pop()?;
        let (line, col) = (token.line, token.column);

        Ok(match token.ttype {
            TType::String(s) => Expr::new(ExprT::String(s), line, col),
            TType::Number(i) => Expr::new(ExprT::Number(i), line, col),
            TType::Ident(i) => Expr::new(ExprT::Identifier(i), line, col),
            TType::Float(f) => Expr::new(ExprT::Float(f), line, col),
            TType::Quote => {
                let following = self.advance(TType::Ident("".to_string()))?;
                let symbol = if let TType::Ident(s) = following.ttype {
                    s
                } else {
                    panic!("Bug: UNEXPECTED_NON_IDENTIFIER");
                };
                Expr::new(ExprT::Symbol(symbol), token.line, token.column)
            }
            TType::LBracket => {
                let mut content = vec![];
                while !self.is_at_end() && self.peek().unwrap().ttype != TType::RBracket {
                    content.push(self.parse_expr()?);
                }

                if !self.is_at_end() {
                    self.advance(TType::RBracket)?;
                }

                Expr::new(ExprT::Array(content), token.line, token.column)
            }
            TType::LParen => {
                let next = self.pop()?;

                match next.ttype {
                    TType::Ident(func) => {
                        let mut args = vec![];
                        while !self.is_at_end() && self.peek().unwrap().ttype != TType::RParen {
                            args.push(self.parse_expr()?);
                        }

                        if !self.is_at_end() {
                            self.advance(TType::RParen)?;
                        }

                        Expr::new(ExprT::Call(func, args), next.line, next.column)
                    }
                    x => return Err(format!("{}:{} | Expected Token of type Identifier, found {}",
                                            line,
                                            col,
                                            x.get_type())),
                }

            }
            x => return Err(format!("{}:{} | Expected Token of type Opening Parenthese, Identifier, String, Number, Quote or Float, found {}",
                                    line,
                                    col,
                                    x.get_type())),
        })
    }
}
