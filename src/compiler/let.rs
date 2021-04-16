use crate::{compiler::Compiler, parser::{Expr, ExprT}};

impl Compiler {
    pub fn let_(&mut self, args: Vec<Expr>) -> Result<String, String> {
       
        if args.len() != 3 {
            return Err(format!("Function `let` takes 3 arguments, but {} arguments were supplied.", args.len()));
        }

        let name = if let ExprT::Var(name) = &args[0].exprt {
            name
        } else {
            return Err(format!("{}:{} | Expected argument of type Var, but found one of type {}.", &args[0].line, &args[0].column, args[0].get_type()));
        };

        let value = self.compile_expr(args[1].clone(), true)?;

        let scope = if let ExprT::Symbol(sym) = &args[2].exprt {
            match sym.as_str() {
                "global" => "g",
                "script" => "s",
                "window" => "w",
                "tab" => "t",
                "buffer" => "b",
                "function" => "l",
                _ => return Err(format!("{}:{} | {}: Unknown symbol.", &args[2].line, &args[2].column, sym)),
            }
        } else {
            return Err(format!("{}:{} | Expected argument of type Symbol, but found one of type {}.", &args[2].line, &args[2].column, args[2].get_type()));
        };

        Ok(format!("let {}:{} = {}", scope, name, value))
    }
}
