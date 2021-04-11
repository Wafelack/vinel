use crate::{VLispResult};

#[derive(Clone, Debug, PartialEq)]
pub enum TType {
    String(String),
    Number(i32),
    Float(f32),
    Ident(String),
    Quote,
    LParen,
    RParen,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub ttype: TType,
    pub column: usize,
    pub line: usize,
}

impl Token {
    pub fn new(ttype: TType, line: usize, column: usize) -> Self {
        Self {
            ttype,
            column,
            line,
        }
    }
}

pub struct Lexer {
    input: String,
    output: Vec<Token>,
    column: usize,
    line: usize,
    current: usize,
    start: usize,
}

impl Lexer {
    pub fn new(input: impl ToString) -> Self {
        Self {
            input: input.to_string(),
            output: vec![],
            column: 0,
            line: 1,
            current: 0,
            start: 0,
        }
    }
    fn advance(&mut self) -> char {
        self.current += 1;
        self.column += 1;
        self.input.chars().nth(self.current - 1).unwrap()
    }
    fn is_at_end(&self) -> bool {
        self.current >= self.input.len()
    }
    fn peek(&self) -> char {
        self.input.chars().nth(self.current).unwrap()
    }
    fn add_token(&mut self, ttype: TType) {
        self.output.push(Token::new(ttype, self.line, self.column));
    }
    fn proc_token(&mut self) -> Result<(), String>{
        let c = self.advance();

        match c {
            '(' => self.add_token(TType::LParen),
            ')' => self.add_token(TType::RParen),
            '"' => self.string()?,
            '\'' => self.add_token(TType::Quote),
            _ => if c.is_digit(10) {
                self.number();
            } else {
                self.identifier();
            }
        }

        Ok(())
    }
    fn identifier(&mut self) {
        let end_chars = vec!['(', ')', ' '];

        while !self.is_at_end() && !end_chars.contains(&self.peek()) {
            self.advance();
        }

        self.add_token(TType::Ident(self.input[self.start..self.current].to_string()));

    }
    fn number(&mut self) {
        while !self.is_at_end() && self.peek().is_digit(10) {
            self.advance();
        }

        if !self.is_at_end() && self.peek() == '.' {
            self.advance();
        }

        while !self.is_at_end() && self.peek().is_digit(10) {
            self.advance();
        }

        let raw = self.input[self.start..self.current].to_string();

        match &raw.parse::<i32>() {
            Ok(z) => self.add_token(TType::Number(*z)),
            _ => match &raw.parse::<f32>() {
                Ok(f) => self.add_token(TType::Float(*f)),
                _ => panic!("Bug: INVALID_NUMBER_SHOULD_BE_VALID"),
            }
        }

    }
    fn string(&mut self) -> Result<(), String> {
        while !self.is_at_end() && self.peek() != '"' {
            if self.peek() == '\n' {
                self.line += 1;
                self.column = 0;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(format!("{}:{} | Unterminated String.", self.line, self.column));
        }

        self.advance();

        self.add_token(TType::String(self.input[self.start + 1..self.current - 1].to_string()));

        Ok(())
    }
    pub fn proc_tokens(&mut self) -> VLispResult<Vec<Token>> {
        let mut errors = vec![];

        while !self.is_at_end() {
            match self.proc_token() {
                Ok(_) => {},
                Err(e) => errors.push(e),
            }
            self.start = self.current;
        }

        if errors.is_empty() {
            Ok(self.output.clone())
        } else {
            Err(errors)
        }
    }
}
