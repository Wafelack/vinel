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
    pub fn source(&mut self, args: Vec<Expr>) -> Result<String, String> {
        if !(1..=2).contains(&args.len()) {
            return Err(format!(
                "Function `source` takes 1 or 2 arguments, but {} arguments were supplied.",
                args.len()
            ));
        }

        let mut normal = false;
        let mut file: Option<String> = None;

        for arg in args.clone() {
            if let ExprT::Symbol(sym) = arg.exprt {
                match sym.as_str() {
                    "normal" => {
                        if normal {
                            return Err(format!(
                                "{}:{} | {}: Duplicated symbol.",
                                arg.line, arg.column, sym
                            ));
                        } else {
                            normal = true;
                        }
                    }
                    _ => {
                        return Err(format!(
                            "{}:{} | {}: Unknown symbol.",
                            arg.line, arg.column, sym
                        ))
                    }
                }
            } else if let ExprT::String(id) = arg.exprt {
                file = Some(id);
            } else if let ExprT::Identifier(id) = arg.exprt {
                file = Some(id);
            } else {
                return Err(format!(
                    "{}:{} | Expected String, Identifier or Symbol, found {}.",
                    arg.line,
                    arg.column,
                    arg.get_type()
                ));
            }
        }

        if file.is_none() {
            let last = args.last().unwrap();

            return Err(format!(
                "{}:{} | File name has not been defined.",
                last.line, last.column
            ));
        }

        Ok(format!(
            "source{} {}",
            if normal { "!" } else { "" },
            file.unwrap()
        ))
    }
}
