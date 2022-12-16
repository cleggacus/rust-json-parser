use std::{fs};

#[derive(Debug, Clone)]
pub enum Punctuation {
    OpenCurly,
    CloseCurly,
    OpenSquare,
    CloseSquare,
    Colon,
    Comma
}

#[derive(Debug, Clone)]
pub enum Token {
    Punctuation(Punctuation),
    String(String),
    Number(f64),
    True,
    False,
    Null
}


pub struct Lexer {
    chars: Vec<char>,
    char_position: usize,
    tokens: Vec<Token>,
    token_position: i64
}

impl Lexer {
    pub fn new(path: &str) -> Self {
        let contents = fs::read_to_string(path)
            .expect("Unable to read file");

        let chars = contents.lines()
            .flat_map(|line| line.chars())
            .collect();

        Self {
            chars,
            char_position: 0,
            tokens: Vec::new(),
            token_position: -1
        }
    }

    pub fn current_token(&mut self) -> Option<Token> {
        if self.token_position < 0 {
            return None;
        } 

        return Some(self.tokens[self.token_position as usize].clone());
    }

    pub fn prev_token(&mut self) -> Option<Token> {
        if self.token_position >= 0 {
            self.token_position -= 1;
        }

        if self.token_position < 0 {
            return None;
        }

        self.current_token()
    }

    pub fn next_token(&mut self) -> Option<Token> {
        if self.token_position < (self.tokens.len() as i64) - 1 {
            self.token_position += 1;
            return self.current_token();
        }

        let token = match self.next_char() {
            None => None,
            Some('{') => Some(Token::Punctuation(Punctuation::OpenCurly)),
            Some('}') => Some(Token::Punctuation(Punctuation::CloseCurly)),
            Some('[') => Some(Token::Punctuation(Punctuation::OpenSquare)),
            Some(']') => Some(Token::Punctuation(Punctuation::CloseSquare)),
            Some(':') => Some(Token::Punctuation(Punctuation::Colon)),
            Some(',') => Some(Token::Punctuation(Punctuation::Comma)),
            Some('t') => return self.collect_true(),
            Some('f') => return self.collect_false(),
            Some('n') => return self.collect_null(),
            Some('"') => return self.collect_string(),
            Some('0'..='9' | '-') => self.collect_number(),
            _ => None
        };

        match token {
            None => None,
            Some(token) => {
                self.tokens.push(token.clone());
                self.token_position += 1;
                self.current_token()
            }
        }

    }

    fn collect_string(&mut self) -> Option<Token> {
        let mut value = String::new();

        while let Some(c) = self.next_char() {
            if c == '"' {
                return Some(Token::String(value));
            }

            value.push(c);
        }

        None
    }

    fn collect_number(&mut self) -> Option<Token> {
        let mut value = String::new();
        let mut has_dot = false;

        self.decrement_char();

        while let Some(c) = self.next_digit() {
            if c == '.' {
                if has_dot {
                    break;
                }

                has_dot = true;
            }

            value.push(c);
        }

        self.decrement_char();

        // println!("{}", value);

        let value: f64 = value.parse().unwrap();

        Some(Token::Number(value))
    }

    fn next_digit(&mut self) -> Option<char> {
        match self.next_char() {
            None => None,
            Some(c) => match c {
                '0'..='9' | '.' | '-' => Some(c),
                _ => None
            }
        }
    }

    fn collect_true(&mut self) -> Option<Token> {
        if self.collect_word("true") {
            return Some(Token::True);
        } 

        None
    }

    fn collect_false(&mut self) -> Option<Token> {
        if self.collect_word("false") {
            return Some(Token::False);
        } 

        None
    }

    fn collect_null(&mut self) -> Option<Token> {
        if self.collect_word("null") {
            return Some(Token::Null);
        } 

        None
    }

    fn collect_word(&mut self, check: &str) -> bool {
        self.decrement_char();

        for check_char in check.chars() {
            if self.next_char() != Some(check_char) {
                return false;
            }
        }

        true
    }

    fn next_char(&mut self) -> Option<char> {
        while !self.is_end() {
            let c = self.current_char();

            self.increment_char();

            if c != ' ' {
                return Some(c)
            }
        }

        return None;
    }

    fn current_char(&mut self) -> char {
        self.chars[self.char_position]
    }

    fn decrement_char(&mut self) {
        if self.char_position > 0 {
            self.char_position -= 1;
        }
    }

    fn increment_char(&mut self) {
        if !self.is_end() {
            self.char_position += 1;
        }
    }

    fn is_end(&mut self) -> bool {
        self.char_position >= self.chars.len()
    }
}
