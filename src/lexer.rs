use std::fs;
use crate::builtin;

#[derive(Debug, PartialEq)]
pub enum LType {
    LParen, // (
    RParen, // )
    LBracket, // <
    RBracket, // >
    Times, // *
    Plus, // +
    Minus, // -
    Div, // /
    Modulo, // %
    Defun,
    Vec,
    Tup,
    While,
    If,
    Else,
    Let,
    Ident(String),
    Str(String),
    Integer(i64),
    Real(f64),
    Builtin(builtin::Builtin)
}
impl LType {
    pub fn get_type(&self) -> String {
        match self {
            Self::LParen => String::from("Opening Parenthese"),
            Self::RParen => String::from("Closing Parenthese"),
            Self::LBracket => String::from("Opening Chevron"),
            Self::RBracket => String::from("Closing Chevron"),
            Self::Times | Self::Plus | Self::Minus | Self::Div | Self::Modulo => String::from("Mathematical operator"),
            Self::Ident(_) => String::from("Identifier"),
            Self::Str(_) => String::from("String"),
            Self::Integer(_) => String::from("Integer"),
            Self::Real(_) => String::from("Real"),
            _ => String::from("Keyword") // Keyword & Builtin
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Token(LType);

impl Token {
    pub fn new(ltype: LType) -> Self {
        Self ( ltype )
    }
}

pub struct Lexer {
    pub input: String,
    output: Vec<Token>,
    line: usize,
    start: usize,
    current: usize,
}

impl Lexer {
    pub fn new(file: String) -> Self {
        Self {
            input: fs::read_to_string(file).expect("Cannot read file."),
            output: vec![],
            line: 1,
            start: 0,
            current: 0
        }
    }

    pub fn debug(&self) -> String {
        format!(
            "output: {:?}, line: {}, start: {}, current_index: {}, current_value: {:?}",
            self.output,
            self.line,
            self.start,
            self.current,
            self.input.chars().nth(self.current)
        )
    }

    pub fn is_eof(&self) -> bool {
        self.current >= self.input.chars().count()
    }

    pub fn peek(&self) -> char {
        self.input.chars().nth(self.current).expect("Hmm.")
    }

    pub fn advance(&mut self) -> char {
        self.current += 1;
        self.input.chars().nth(self.current - 1).unwrap()
    }

    pub fn lex_one(&mut self) {
        let cc = self.advance();

        match cc {
            ' ' => {}
            '>' => self.add_token(LType::RBracket),
            '<' => self.add_token(LType::LBracket),
            '(' => self.add_token(LType::LParen),
            ')' => self.add_token(LType::RParen),
            '"' => {
                    self.start += 1;
                    self.string();
                }
            '-' => self.add_token(LType::Minus),
            '+' => self.add_token(LType::Plus),
            '*' => self.add_token(LType::Times),
            '/' => self.add_token(LType::Div),
            '%' => self.add_token(LType::Modulo),
            '\n' => self.line += 1,
            x => if x.is_numeric() {
                    self.number();
                } else {
                    self.identifier();
                }
        }
    }

    pub fn add_token(&mut self, ltype: LType) {
        self.output.push(Token(ltype));
    }

    pub fn string(&mut self) {
        'outer: while !self.is_eof() {
            match self.peek() {
                '"' => break 'outer,
                '\n' => self.line += 1,
                _ => (),
            }
            self.advance();
        }
        if self.is_eof()  {
            panic!("Unlimited string.");
        }

        let str = self.input[self.start..self.current].to_string();
        self.current += 1; // Spell of quotation marks
        self.add_token(LType::Str(str));
    }

    pub fn number(&mut self) {
        let stop = vec![')', '\n', ' '];

        while !self.is_eof() && !stop.contains(&self.peek()) { self.advance(); }

        let num = self.input[self.start..self.current].to_string();
        
        if num.chars().last().unwrap() == '.' {
            panic!("Expected a decimal, but nothing found.");
        }

        let value = match num.parse::<i64>() {
            Ok(v) => v as f64,
            Err(_) => num.parse::<f64>().unwrap()
        };

        // Is negative
        let value = if self.input.chars().nth(self.start - 1).unwrap() == '-' {
            self.output.pop();
            -value
        } else { value };

        self.add_token(if value.fract() == 0.0 {
            LType::Integer(value as i64)
        } else {
            LType::Real(value)
        });
    }

    pub fn identifier(&mut self) {
        let stop = vec!['(',')', '\n', ' ', '"'];
        while !self.is_eof() && !stop.contains(&self.peek()) { self.advance(); }
        let ident = self.input[self.start..self.current].to_string();
        
        match &ident as &str {
            // Keyword
            "defun" => self.add_token(LType::Defun),
            "vec" => self.add_token(LType::Vec),
            "tup" => self.add_token(LType::Tup),
            "if" => self.add_token(LType::If),
            "else" => self.add_token(LType::Else),
            "while" => self.add_token(LType::While),
            "let" => self.add_token(LType::Let),

            // Builtin
            "put" => self.add_token(LType::Builtin(builtin::Builtin::Put)),
            "get" => self.add_token(LType::Builtin(builtin::Builtin::Get)),
            "type" => self.add_token(LType::Builtin(builtin::Builtin::Type)),
            "len" => self.add_token(LType::Builtin(builtin::Builtin::Len)),
            "panic" => self.add_token(LType::Builtin(builtin::Builtin::Panic)),
            "push" => self.add_token(LType::Builtin(builtin::Builtin::Push)),
            "pop" => self.add_token(LType::Builtin(builtin::Builtin::Pop)),

            _ => self.add_token(LType::Ident(ident))
        }
    }

    pub fn lex(&mut self) {
        while !self.is_eof() {
            self.lex_one();
            self.start = self.current;
        }
    }
}