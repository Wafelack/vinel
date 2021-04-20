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
    pub fn get(&mut self, args: Vec<Expr>) -> Result<String, String> {
        if !(0..=2).contains(&args.len()) {
            return Err(format!(
                "Function `get` takes 0, 1 or 2 arguments, but {} arguments were supplied.",
                args.len()
            ));
        }

        let (option, newline, termcap, all) = if args.len() == 0 {
            (None, false, false, false)
        } else if args.len() == 1 {
            if let ExprT::Identifier(option) = &args[0].exprt {
                (Some(option), false, false, false)
            } else if let ExprT::Symbol(sym) = &args[0].exprt {
                match sym.as_str() {
                    "all" => (None, false, false, true),
                    "termcap" => (None, false, true, false),
                    "newline" => (None, true, false, false),
                    _ => {
                        return Err(format!(
                            "{}:{} | {}: Unknown symbol.",
                            &args[0].line, &args[0].column, sym
                        ))
                    }
                }
            } else {
                return Err(format!(
                    "{}:{} | Expected Symbol or Identifier, found {}.",
                    &args[0].line,
                    &args[0].column,
                    args[0].get_type()
                ));
            }
        } else {
            let (mut newline, mut termcap, mut all) = if let ExprT::Symbol(sym) = &args[0].exprt {
                match sym.as_str() {
                    "all" => (false, false, true),
                    "termcap" => (false, true, false),
                    "newline" => (true, false, false),
                    _ => {
                        return Err(format!(
                            "{}:{} | {}: Unknown symbol.",
                            &args[0].line, &args[0].column, sym
                        ))
                    }
                }
            } else {
                return Err(format!(
                    "{}:{} | Expected Symbol, found {}.",
                    &args[0].line,
                    &args[0].column,
                    args[0].get_type()
                ));
            };

            if let ExprT::Symbol(sym) = &args[1].exprt {
                match sym.as_str() {
                    "all" => {
                        if all {
                            return Err(format!(
                                "{}:{} | all: Symbol is already present.",
                                &args[1].line, args[1].column
                            ));
                        } else {
                            all = true
                        }
                    }
                    "termcap" => {
                        if termcap {
                            return Err(format!(
                                "{}:{} | termcap: Symbol is already present.",
                                &args[1].line, args[1].column
                            ));
                        } else {
                            termcap = true
                        }
                    }
                    "newline" => {
                        if newline {
                            return Err(format!(
                                "{}:{} | newline: Symbol is already present.",
                                &args[1].line, args[1].column
                            ));
                        } else {
                            newline = true
                        }
                    }
                    _ => {
                        return Err(format!(
                            "{}:{} | {}: Unknown symbol.",
                            &args[1].line, &args[1].column, sym
                        ))
                    }
                }
            } else {
                return Err(format!(
                    "{}:{} | Expected Symbol, found {}.",
                    &args[1].line,
                    &args[1].column,
                    args[1].get_type()
                ));
            }

            (None, newline, termcap, all)
        };

        if termcap && all {
            return Err(format!(
                "Symbols `termcap` and `all` cannot be present simultaneously."
            ));
        }

        Ok(format!(
            "set{}{}",
            if newline { "!" } else { "" },
            if all {
                " all".to_string()
            } else if termcap {
                " termcap".to_string()
            } else if option.is_some() {
                format!(" {}?", option.unwrap())
            } else {
                "".to_string()
            }
        ))
    }
}
