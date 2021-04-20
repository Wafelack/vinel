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
    pub fn set(&mut self, args: Vec<Expr>) -> Result<String, String> {
        if !(1..=2).contains(&args.len()) {
            return Err(format!(
                "Function `set` takes 1 or 2 arguments, but {} arguments were supplied.",
                args.len()
            ));
        }

        if args.len() == 1 {
            if let ExprT::Identifier(opt) = &args[0].exprt {
                Ok(format!("set {}", opt))
            } else {
                Err(format!(
                    "{}:{} | Expected Identifier, found {}.",
                    &args[0].line,
                    &args[0].column,
                    &args[0].get_type()
                ))
            }
        } else {
            let option = if let ExprT::Identifier(opt) = &args[0].exprt {
                opt
            } else {
                return Err(format!(
                    "{}:{} | Expected Identifier, found {}.",
                    &args[0].line,
                    &args[0].column,
                    &args[0].get_type()
                ));
            };

            Ok(if let ExprT::Symbol(sym) = &args[1].exprt {
                match sym.as_str() {
                    "toggle" => format!("set inv{}", option),
                    "reset" => format!("set {}&", option),
                    "vi" => format!("set {}&vi", option),
                    "vim" => format!("set {}&vim", option),
                    "off" => format!("set no{}", option),
                    _ => {
                        return Err(format!(
                            "{}:{} | {}: Unknown symbol.",
                            &args[1].line, &args[1].column, sym
                        ))
                    }
                }
            } else {
                format!(
                    "set {}={}",
                    option,
                    self.compile_expr(args[1].clone(), true)?
                )
            })
        }
    }
}
