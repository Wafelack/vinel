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
    parser::{Expr, ExprT},
};

impl Compiler {
    pub fn cond(&mut self, args: Vec<Expr>) -> Result<String, String> {
        if args.len() < 2 {
            return Err(format!(
                "Function `get` takes 2 or more arguments, but {} arguments were supplied.",
                args.len()
            ));
        }

        let mut pairs = vec![];

        for arg in args {
            if let ExprT::Array(content) = arg.exprt {
                if content.len() < 2 {
                    return Err(format!(
                        "{}:{} | Expected 2 or more values in the Array, but {} values were found.",
                        arg.line,
                        arg.column,
                        content.len()
                    ));
                } else {
                    let conditional = self.compile_expr(content[0].clone(), false)?;
                    let mut todo = String::new();

                    for part in content.iter().skip(1) {
                        let compiled = self.compile_expr(part.clone(), false)?;
                        todo.push_str(&format!("{}\n", compiled));
                    }
                    pairs.push((conditional, todo));
                }
            } else {
                return Err(format!(
                    "{}:{} | Expected an Array, found a {}.",
                    arg.line,
                    arg.column,
                    arg.get_type()
                ));
            }
        }

        let mut to_ret = String::new();

        for (idx, pair) in pairs.into_iter().enumerate() {
            let cond = if idx == 0 {
                "if"
            } else if pair.0.as_str() == "else" {
                "else"
            } else {
                "elseif"
            };

            to_ret.push_str(&format!(
                "{} {}\n",
                cond,
                if cond == "else" { "" } else { &pair.0 }
            ));
            to_ret.push_str(pair.1.as_str());
        }
        to_ret.push_str("endif");

        Ok(to_ret)
    }
}
