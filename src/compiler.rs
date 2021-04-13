use crate::{parser::{ExprT, Expr}, VLispResult};

pub struct Compiler {
    input: Vec<Expr>,
    output: String,
}

impl Compiler {
    pub fn new(input: Vec<Expr>) -> Self {
        Self {
            input,
            output: "".to_string(),
        }
    }
    pub fn compile_expr(&mut self, expr: Expr) -> Result<String, String> {
           
        let (exprt, line, column) = (expr.exprt, expr.line, expr.column);

        match exprt {
            ExprT::String(s) => Ok(format!("\"{}\"", s)),
            ExprT::Number(i) => Ok(format!("{}", i)),
            ExprT::Float(f) => Ok(format!("{}", f)),
            ExprT::Var(s) => Ok(format!("{}", s)),
            ExprT::Call(function, arguments) => todo!(),
            ExprT::Symbol(_) => Err(format!("{}:{} | Expected Variable, Function Call, Float, Number or String, found Symbol.", line, column))
        }

    }
    pub fn compile(&mut self) -> VLispResult<String> {
        let mut errors = vec![];

        for expr in self.input {
            let to_push = match self.compile_expr(expr) {
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
