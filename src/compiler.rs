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
use crate::{parser::{ExprT, Expr}, VLispResult};

mod map;
mod operator;
mod r#let;
mod set;
mod get;
mod defun;
mod cond;
mod call;
mod source;
mod edit;
mod mark;
mod colorscheme;
mod any;
mod raw;
mod tabs;

pub struct Compiler {
    input: Vec<Expr>,
    output: String,
}

fn adapt(out: String, in_expr: bool) -> Result<String, String> {
    Ok(format!("{}{}{}", if in_expr { ":" } else { "" }, out, if in_expr { "<CR>" } else { "" }))

}

impl Compiler {
    pub fn new(input: Vec<Expr>) -> Self {
        Self {
            input,
            output: "".to_string(),
        }
    }
    pub fn compile_expr(&mut self, expr: Expr, in_expr: bool) -> Result<String, String> {

        let (exprt, line, column) = (expr.exprt, expr.line, expr.column);

        match exprt {
            ExprT::String(s) => Ok(format!("\"{}\"", s)),
            ExprT::Number(i) => Ok(format!("{}", i)),
            ExprT::Float(f) => Ok(format!("{}", f)),
            ExprT::Identifier(s) => Ok(format!("{}", s)),
            ExprT::Array(exprs) => {
                let mut new = "[".to_string();
                let length = exprs.len();
                for (idx, expr)in exprs.into_iter().enumerate() {
                    new.push_str(&self.compile_expr(expr, false)?);
                    if idx != length - 1 {
                        new.push(',');
                    }
                }
                new.push(']');
                Ok(new)
            }
            ExprT::Call(function, arguments) => match function.as_str() {
                "map" => adapt(self.map(arguments)?, in_expr),
                "let" => adapt(self.let_(arguments)?, in_expr),
                "get" => adapt(self.get(arguments)?, in_expr),
                "set" => adapt(self.set(arguments)?, in_expr),
                "defun" => adapt(self.defun(arguments)?, in_expr),
                "+" | "-" | "*" | "/" | "." | "==#" | "==?" | "==" | "is" | "isnot" | "is#" | "is?" | ">=#" | ">=?" | "and" | "or" | ">" | ">=" | "<" | "<=" | "=~" | "!~" | "!=" => self.operator(function.as_str(), arguments),
                "cond" => adapt(self.cond(arguments)?, in_expr),
                "call" => adapt(self.call(arguments)?, in_expr),
                "source" => adapt(self.source(arguments)?, in_expr),
                "edit" => adapt(self.edit(arguments)?, in_expr),
                "top" => Ok("gg".to_string()),
                "bottom" => Ok("G".to_string()),
                "esc" => Ok("<ESC>".to_string()),
                "ret" => Ok("<CR>".to_string()),
                "mark" => self.mark(arguments),
                "goto" => self.goto(arguments),
                "gotab" => self.gotab(arguments),
                "colorscheme" => self.colorscheme(arguments),
                "raw" => self.raw(arguments),
                x => self.any(x, arguments, in_expr),
            }
            ExprT::Symbol(_) => Err(format!("{}:{} | Expected Identifier, Function Call, Float, Number or String, found Symbol.", line, column))
        }

    }
    pub fn compile(&mut self) -> VLispResult<String> {
        let mut errors = vec![];

        for expr in self.input.clone() {
            let to_push = match self.compile_expr(expr, false) {
                Ok(s) => s,
                Err(e) => {
                    errors.push(e);
                    continue;
                },
            };

            self.output.push_str(&format!("{}\n", to_push));
        }

        if errors.is_empty() {
            Ok(self.output.clone())
        } else {
            Err(errors)
        }
    }
}
