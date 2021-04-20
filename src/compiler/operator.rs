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
use crate::{compiler::Compiler, parser::Expr};

impl Compiler {
    pub fn operator(&mut self, operator: &str, args: Vec<Expr>) -> Result<String, String> {
        if args.len() != 2 {
            return Err(format!(
                "Function `{}` takes 2 arguments, but {} arguments were supplied.",
                operator,
                args.len()
            ));
        }

        let operator = if operator == "and" {
            "&&"
        } else if operator == "or" {
            "||"
        } else {
            operator
        };

        let lhs = self.compile_expr(args[0].clone(), false)?;
        let rhs = self.compile_expr(args[1].clone(), false)?;

        Ok(format!("({} {} {})", lhs, operator, rhs))
    }
}
