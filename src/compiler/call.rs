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
    pub fn call(&mut self, args: Vec<Expr>) -> Result<String, String> {

        if args.len() < 1  {
            return Err(format!("Function `call` takes 1 or more arguments, but 0 arguments were supplied."));
        }

        let fname = if let ExprT::Identifier(id) = &args[0].exprt {
           id.to_string()
        } else {
            return Err(format!("{}:{} | Expected an Identifier, found a {}.", &args[0].line, &args[0].column, &args[0].get_type()));
        };

        let mut fn_args = String::new();
        let length = args.len();
        for (idx, arg) in args.into_iter().skip(1).enumerate() {
            fn_args.push_str(&self.compile_expr(arg, false)?);
            if idx != length - 2 {
                fn_args.push(',');
            }
        }

        Ok(format!("call {}({})", fname, fn_args))
    }
}
