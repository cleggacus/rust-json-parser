use std::{fs};

#[derive(Debug)]
pub enum Punctuation {
    OpenCurly,
    CloseCurly,
    OpenSquare,
    CloseSquare,
    Colon,
    Comma
}

#[derive(Debug)]
pub enum Token {
    Punctuation(Punctuation),
    String(String),
    Number(f64),
    True,
    False,
    Null
}


pub struct Lexer {
    pub chars: Vec<char>,
    char_position: usize
}

impl Lexer {
    pub fn new(path: &str) -> Self {
        let contents = fs::read_to_string(path)
            .expect("Unable to read file");

        let chars = contents.lines()
            .flat_map(|line| line.chars())
            .collect();

        return Self {
            chars,
            char_position: 0
        };
    }

    pub fn next_token(&mut self) -> Option<Token> {
        match self.next_char() {
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
            Some('1'..='9' | '-') => self.collect_number(),
            _ => None
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

        let value: f64 = value.parse().unwrap();

        Some(Token::Number(value))
    }

    fn next_digit(&mut self) -> Option<char> {
        match self.next_char() {
            None => None,
            Some(c) => match c {
                '0'..='9' | '.' => Some(c),
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
