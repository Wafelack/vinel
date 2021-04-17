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
use crate::{compiler::Compiler, parser::{Expr, ExprT}};

impl Compiler {
    pub fn let_(&mut self, args: Vec<Expr>) -> Result<String, String> {
       
        if args.len() != 3 {
            return Err(format!("Function `let` takes 3 arguments, but {} arguments were supplied.", args.len()));
        }

        let name = if let ExprT::Identifier(name) = &args[0].exprt {
            name
        } else {
            return Err(format!("{}:{} | Expected argument of type Identifier, but found one of type {}.", &args[0].line, &args[0].column, args[0].get_type()));
        };

        let value = self.compile_expr(args[1].clone(), true)?;

        let scope = if let ExprT::Symbol(sym) = &args[2].exprt {
            match sym.as_str() {
                "global" => "g",
                "script" => "s",
                "window" => "w",
                "tab" => "t",
                "buffer" => "b",
                "function" => "l",
                _ => return Err(format!("{}:{} | {}: Unknown symbol.", &args[2].line, &args[2].column, sym)),
            }
        } else {
            return Err(format!("{}:{} | Expected argument of type Symbol, but found one of type {}.", &args[2].line, &args[2].column, args[2].get_type()));
        };

        Ok(format!("let {}:{} = {}", scope, name, value))
    }
}
