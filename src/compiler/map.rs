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
    pub fn map(&mut self, args: Vec<Expr>) -> Result<String, String> {
        if args.len() < 1 {
            return Err(format!(
                "Function `map` takes one argument or more, but 0 arguments were supplied."
            ));
        }

        let mut recursive = false;
        let mut to_do = "".to_string();
        let mut options = "".to_string();
        let mut mode = "any".to_string();
        let sequence = if let ExprT::String(s) = &args[0].exprt {
            s
        } else {
            return Err(format!(
                "{}:{} | Expected String, found {}.",
                &args[0].line,
                &args[0].column,
                args[0].get_type()
            ));
        };

        for arg in args.iter().skip(1) {
            if let ExprT::Symbol(symbol) = &arg.exprt {
                match symbol.as_str() {
                    "normal" | "visual" | "insert" => mode = symbol.to_string(),
                    "buffer" | "nowait" | "silent" | "special" | "script" | "expr" | "unique" => {
                        options.push_str(&format!("<{}>", symbol))
                    }
                    "recursive" => recursive = true,
                    _ => {
                        return Err(format!(
                            "{}:{} | {}: Unknown symbol.",
                            arg.line, arg.column, symbol
                        ))
                    }
                }
            } else {
                to_do.push_str(&self.compile_expr(arg.clone(), true)?);
            }
        }

        Ok(format!(
            "{}{}map {} {} {}",
            match mode.as_str() {
                "any" => "",
                "normal" => "n",
                "insert" => "i",
                "visual" => "v",
                _ => panic!("UNEXPECTED_INVALID_MODE"),
            },
            if recursive { "" } else { "nore" },
            options,
            sequence,
            to_do
        ))
    }
}
