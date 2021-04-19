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
    pub fn edit(&mut self, args: Vec<Expr>) -> Result<String, String> {

        if args.len() > 2 {
            return Err(format!("Function `edit` takes at most 2 arguments, but {} arguments were supplied.", args.len()));
        }

        let mut discard = false;
        let mut file: Option<String> = None;

        for arg in args {
            if let ExprT::Symbol(sym) = arg.exprt {
                match sym.as_str() {
                    "discard" => if discard {
                        return Err(format!("{}:{} | {}: Duplicated symbol.", arg.line, arg.column, sym));
                    } else {
                        discard = true;
                    }
                    _ => return Err(format!("{}:{} | {}: Unknown symbol.", arg.line, arg.column, sym)),
                }
            } else if let ExprT::String(id) = arg.exprt {
                file = Some(id)
            } else if let ExprT::Identifier(id) = arg.exprt {
                file = Some(id);
            } else {
                return Err(format!("{}:{} | Expected String, Identifier or Symbol, found {}.", arg.line, arg.column, arg.get_type()));
            }
        }
        
        Ok(format!("edit{}{}", if discard { "!" } else { "" }, if file.is_some() { format!(" {}", file.unwrap()) } else { "".to_string() }))
    }
}
