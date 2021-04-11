use crate::{Token, TType, VLispResult};
use std::mem::discriminant;

#[derive(Clone,Debug,  PartialEq)]
pub enum ExprT {
    String(String),
    Number(i32),
    Float(f32),
    Symbol(String),
    Call(String, Vec<Expr>),
    Var(String),
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
    pub fn parse(&mut self) -> VLispResult<Vec<Expr>> {
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
            TType::Ident(i) => Expr::new(ExprT::Var(i), line, col),
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
