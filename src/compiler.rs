use crate::{parser::{ExprT, Expr}, VLispResult};

mod map;
mod r#let;
mod get;

pub struct Compiler {
    input: Vec<Expr>,
    output: String,
}

fn adapt(out: String, in_expr: bool) -> Result<String, String> {
    Ok(format!("{}{}{}", if in_expr { ":" } else { "" }, out, if in_expr { "<CR>" } else { "" }))

}

impl Compiler {
    pub fn new(input: Vec<Expr>) -> Self {
        Self {
            input,
            output: "".to_string(),
        }
    }
    pub fn compile_expr(&mut self, expr: Expr, in_expr: bool) -> Result<String, String> {

        let (exprt, line, column) = (expr.exprt, expr.line, expr.column);

        match exprt {
            ExprT::String(s) => Ok(format!("\"{}\"", s)),
            ExprT::Number(i) => Ok(format!("{}", i)),
            ExprT::Float(f) => Ok(format!("{}", f)),
            ExprT::Var(s) => Ok(format!("{}", s)),
            ExprT::Call(function, arguments) => match function.as_str() {
                "map" => adapt(self.map(arguments)?, in_expr),
                "let" => adapt(self.let_(arguments)?, in_expr),
                "get" => adapt(self.get(arguments)?, in_expr),
                _ => todo!(),
            }
            ExprT::Symbol(_) => Err(format!("{}:{} | Expected Variable, Function Call, Float, Number or String, found Symbol.", line, column))
        }

    }
    pub fn compile(&mut self) -> VLispResult<String> {
        let mut errors = vec![];

        for expr in self.input.clone() {
            let to_push = match self.compile_expr(expr, false) {
                Ok(s) => s,
                Err(e) => {
                    errors.push(e);
                    continue;
                },
            };

            self.output.push_str(&format!("{}\n", to_push));
        }

        if errors.is_empty() {
            Ok(self.output.clone())
        } else {
            Err(errors)
        }
    }
}
