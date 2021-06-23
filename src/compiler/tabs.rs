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
    pub fn gotab(&mut self, args: Vec<Expr>) -> Result<String, String> {
        if !(0..=1).contains(&args.len()) {
            return Err(format!(
                "Function `gotab` takes 0 or 1 argument, but {} arguments were supplied.",
                args.len()
            ));
        }

        let number = if args.len() == 1 {
            if let ExprT::Number(n) = args[0].exprt {
                Some(n)
            } else {
                return Err(format!(
                    "{}:{} | Expected a Number, found a {}.",
                    args[0].line,
                    args[0].column,
                    args[0].get_type()
                ));
            }
        } else {
            None
        };

        Ok(format!(
            "{}gt",
            if number.is_some() {
                format!("{}", number.unwrap())
            } else {
                format!("")
            }
        ))
    }
}
