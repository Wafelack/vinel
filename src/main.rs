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
mod compiler;
mod lexer;
mod parser;
#[cfg(test)]
mod tests;

use clap::{App, Arg};
use compiler::Compiler;
use lexer::Lexer;
use parser::Parser;
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
    process::exit,
};

const ERROR: &str = "\x1b[0;31merror\x1b[0m:";

fn try_main() -> VLispResult<()> {
    let matches = App::new("GVLC")
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .about("The Gentle Vim Lisp Compiler")
        .arg(
            Arg::with_name("file")
                .required(true)
                .takes_value(true)
                .index(1)
                .help("The source file to compile."),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .takes_value(true)
                .help("The name of the output file."),
        )
        .get_matches();

    let content = match fs::read_to_string(matches.value_of("file").unwrap()) {
        Ok(c) => c,
        Err(e) => return Err(vec![format!("Failed to read file: {}.", e)]),
    };

    let tokens = Lexer::new(&content).proc_tokens()?;
    let ast = Parser::new(tokens).parse()?;
    let output = Compiler::new(ast).compile()?;

    let output_file = matches
        .value_of("output")
        .and_then(|v| Some(v.to_string()))
        .unwrap_or(format!(
            "{}.vim",
            Path::new(matches.value_of("file").unwrap())
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
        ));

    match {
        match File::create(output_file) {
            Ok(f) => f,
            Err(e) => return Err(vec![format!("Failed to open file for writing: {}.", e)]),
        }
        .write_all(output.as_bytes())
    } {
        Ok(_) => {}
        Err(e) => return Err(vec![format!("Failed to write file: {}.", e)]),
    }

    Ok(())
}

fn main() {
    match try_main() {
        Ok(_) => {}
        Err(errors) => {
            for error in &errors {
                eprintln!("{} {}", ERROR, error);
            }

            eprintln!(
                "\n{} Aborting due to {} previous errors.",
                ERROR,
                errors.len()
            );
            exit(1);
        }
    }
}

pub type VLispResult<T> = std::result::Result<T, Vec<String>>;
