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
    pub fn raw(&mut self, args: Vec<Expr>) -> Result<String, String> {
        let mut to_write = String::new();

        for arg in args {
            if let ExprT::String(part) = arg.exprt {
                to_write.push_str(&format!("{}\n", part));
            } else {
                return Err(format!(
                    "{}:{} | Expected a String, found a {}.",
                    arg.line,
                    arg.column,
                    arg.get_type()
                ));
            }
        }

        Ok(to_write)
    }
}
