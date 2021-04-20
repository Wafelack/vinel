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
    compiler::Compiler,
    parser::{Expr, ExprT},
};

impl Compiler {
    pub fn mark(&mut self, args: Vec<Expr>) -> Result<String, String> {
        if args.len() != 1 {
            return Err(format!(
                "Function `mark` takes 1 argument, but {} arguments were supplied.",
                args.len()
            ));
        }

        let mark = if let ExprT::Identifier(m) = &args[0].exprt {
            m
        } else {
            return Err(format!(
                "{}:{} | Expected an Identifier, found a {}.",
                args[0].line,
                args[0].column,
                args[0].get_type()
            ));
        };

        if mark.len() != 1 {
            return Err(format!("{}:{} | Mark name should be only 1 character long, found a {} characters long one.", args[0].line, args[0].column, mark.len()));
        }

        Ok(format!("m{}", mark))
    }

    pub fn goto(&mut self, args: Vec<Expr>) -> Result<String, String> {
        if args.len() != 1 {
            return Err(format!(
                "Function `goto` takes 1 argument, but {} arguments were supplied.",
                args.len()
            ));
        }

        let mark = if let ExprT::Identifier(m) = &args[0].exprt {
            m
        } else {
            return Err(format!(
                "{}:{} | Expected an Identifier, found a {}.",
                args[0].line,
                args[0].column,
                args[0].get_type()
            ));
        };

        if mark.len() != 1 {
            return Err(format!("{}:{} | Mark name should be only 1 character long, found a {} characters long one.", args[0].line, args[0].column, mark.len()));
        }

        Ok(format!("`{}", mark))
    }
}
