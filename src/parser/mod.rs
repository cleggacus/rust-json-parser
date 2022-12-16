use self::{lexer::{Lexer, Token, Punctuation}, node::Value, node::Node};

mod lexer;
pub mod node;

pub struct Parser {
    lexer: Lexer
}

impl Parser {
    pub fn new(path: &str) -> Self {
        Self {
            lexer: Lexer::new(path)
        }
    }

    pub fn parse_value(&mut self) -> Option<Value> {
        match self.lexer.next_token() {
            Some(Token::Punctuation(Punctuation::OpenCurly)) => {
                self.lexer.prev_token(); 
                self.parse_object()
            },
            Some(Token::Punctuation(Punctuation::OpenSquare)) => {
                self.lexer.prev_token(); 
                self.parse_array()
            },
            Some(Token::Null) => Some(Value::Null),
            Some(Token::True) => Some(Value::Boolean(true)),
            Some(Token::False) => Some(Value::Boolean(false)),
            Some(Token::String(value)) => Some(Value::String(value)),
            Some(Token::Number(value)) => Some(Value::Number(value)),
            _ => None
        }
    }

    fn parse_array(&mut self) -> Option<Value> {
        let mut nodes: Vec<Value> = Vec::new();

        while let Some(token) = self.lexer.next_token() {
            match token {
                Token::Punctuation(Punctuation::CloseSquare) => break,
                Token::Punctuation(Punctuation::Comma) |
                Token::Punctuation(Punctuation::OpenSquare) => {
                    let node = self.parse_value();

                    if node.is_none() {
                        return None;
                    } else {
                        nodes.push(node.unwrap());
                    }
                },
                _ => return None
            }
        }

        Some(Value::Array(nodes))
    }

    fn parse_object(&mut self) -> Option<Value> {
        let mut nodes: Vec<Node> = Vec::new();

        while let Some(token) = self.lexer.next_token() {
            match token {
                Token::Punctuation(Punctuation::CloseCurly) => {
                    break; 
                },
                Token::Punctuation(Punctuation::Comma) |
                Token::Punctuation(Punctuation::OpenCurly) => {
                    let node = self.parse_node();

                    if node.is_none() {
                        return None;
                    } else {
                        nodes.push(node.unwrap());
                    }
                },
                _ => return None
            }
        }

        Some(Value::Object(nodes))
    }

    fn parse_node(&mut self) -> Option<Node> {
        let key = match self.lexer.next_token() {
            Some(Token::String(key)) => Some(String::from(key)),
            _ => {
                None
            }
        };

        if key.is_none() {
            return None;
        }

        let colon = match self.lexer.next_token() {
            Some(Token::Punctuation(Punctuation::Colon)) => true,
            _ => false
        };

        if !colon {
            return None;
        }

        let value = self.parse_value();

        if value.is_none() {
            return None;
        }

        let node = Node { 
            key: key.unwrap(), 
            value: value.unwrap() 
        };

        Some(node)

    }
}
