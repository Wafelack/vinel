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
    pub fn dict(&mut self, args: Vec<Expr>) -> Result<String, String> {

        if args.len() % 2 != 0 {
            return Err(format!("Function `dict` takes an even amount of arguments, but {} arguments were supplied.", args.len()));
        }

        let mut pairs = vec![];
        for idx in (0..args.len()).step_by(2) {
            pairs.push((self.compile_expr(args[idx].clone(), false)?, self.compile_expr(args[idx + 1].clone(), false)?));
        }


        Ok(format!("{{ {} }}", pairs.into_iter().map(|(k, v)| format!("{}:{}", k, v)).collect::<Vec<_>>().join(", ")))

    }
}
