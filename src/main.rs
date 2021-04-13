mod lexer;
mod parser;
mod compiler;
mod tests;

fn main() {
    println!("Hello, world!");
}

pub type VLispResult<T> = std::result::Result<T, Vec<String>>;
