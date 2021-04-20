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
    pub fn defun(&mut self, args: Vec<Expr>) -> Result<String, String> {
        if args.len() < 1 {
            return Err(format!(
                "Function `defun` takes 1 or more arguments, but 0 arguments were supplied."
            ));
        }

        let mut script = false;
        let mut no_overwrite = false;
        let mut abort = false;
        let mut name: Option<String> = None;
        let mut func_args = vec![];
        let mut body = "".to_string();

        for arg in args.clone() {
            if let ExprT::Symbol(sym) = &arg.exprt {
                match sym.as_str() {
                    "script" => {
                        if script {
                            return Err(format!(
                                "{}:{} | {}: Duplicated symbol.",
                                arg.line, arg.column, sym
                            ));
                        } else {
                            script = true;
                        }
                    }
                    "no-overwrite" => {
                        if no_overwrite {
                            return Err(format!(
                                "{}:{} | {}: Duplicated symbol.",
                                arg.line, arg.column, sym
                            ));
                        } else {
                            no_overwrite = true;
                        }
                    }
                    "abort" => {
                        if abort {
                            return Err(format!(
                                "{}:{} | {}: Duplicated symbol.",
                                arg.line, arg.column, sym
                            ));
                        } else {
                            abort = true;
                        }
                    }
                    _ => {
                        return Err(format!(
                            "{}:{} | {}: Unknown symbol.",
                            arg.line, arg.column, sym
                        ))
                    }
                }
            } else if name.is_none() {
                if let ExprT::Call(n, args) = arg.exprt {
                    name = Some(n.to_string());
                    for arg in args {
                        if let ExprT::Identifier(arg) = arg.exprt {
                            func_args.push(arg);
                        } else {
                            return Err(format!(
                                "{}:{} | Expected an Identifier, found {}.",
                                arg.line,
                                arg.column,
                                arg.get_type()
                            ));
                        }
                    }
                } else {
                    return Err(format!(
                        "{}:{} | Expected name and arguments definiton (Function Call), found {}.",
                        arg.line,
                        arg.column,
                        arg.get_type()
                    ));
                }
            } else {
                body.push_str(&format!("{}\n", &self.compile_expr(arg.clone(), false)?));
            }
        }
        if name.is_none() {
            let last = args.last().unwrap();
            return Err(format!(
                "{}:{} | Function name has not been defined.",
                last.line, last.column
            ));
        }

        Ok(format!(
            "function{} {}({}){}\n{}endfunction",
            if no_overwrite { "" } else { "!" },
            if script {
                format!("s:{}", name.unwrap())
            } else {
                name.unwrap()
            },
            func_args.join(", "),
            if abort { " abort" } else { "" },
            body
        ))
    }
}
