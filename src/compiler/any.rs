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
    pub fn any(&mut self, name: &str, args: Vec<Expr>, in_expr: bool) -> Result<String, String> {
        let mut command = false;
        let mut fn_args = vec![];

        for arg in args {
            if let ExprT::Symbol(sym) = arg.exprt {
                match sym.as_str() {
                    "command" => {
                        if command {
                            return Err(format!(
                                "{}:{} | {}: Duplicated symbol.",
                                arg.line, arg.column, sym
                            ));
                        } else {
                            command = true;
                        }
                    }
                    _ => {
                        return Err(format!(
                            "{}:{} | {}: Unknown symbol.",
                            arg.line, arg.column, sym
                        ))
                    }
                }
            } else {
                fn_args.push(self.compile_expr(arg, false)?);
            }
        }

        Ok(format!(
            "{}{}{}",
            if in_expr { ":" } else { "" },
            name,
            if command {
                format!(" {}", fn_args.join(" "))
            } else {
                format!("({})", fn_args.join(", "))
            }
        ))
    }
}
