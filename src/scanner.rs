use std::collections::HashMap;

use crate::token::{Token, TokenType};

pub struct Lexer<'a> {
    source: &'a str,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: u32,
    keywords: HashMap<String, TokenType>,
}

impl Lexer<'_> {
    pub fn new(src: &String) -> Lexer {
        let mut keywords = HashMap::new();
        keywords.insert("et".to_string(), TokenType::AND);
        keywords.insert("class".to_string(), TokenType::CLASS);
        keywords.insert("sinon".to_string(), TokenType::ELSE);
        keywords.insert("faux".to_string(), TokenType::FALSE);
        keywords.insert("for".to_string(), TokenType::FOR);
        keywords.insert("fonction".to_string(), TokenType::FUN);
        keywords.insert("si".to_string(), TokenType::IF);
        keywords.insert("VIDE".to_string(), TokenType::NIL);
        keywords.insert("ou".to_string(), TokenType::OR);
        keywords.insert("ecrire".to_string(), TokenType::PRINT);
        keywords.insert("retourner".to_string(), TokenType::RETURN);
        keywords.insert("super".to_string(), TokenType::SUPER);
        keywords.insert("this".to_string(), TokenType::THIS);
        keywords.insert("vrai".to_string(), TokenType::TRUE);
        keywords.insert("variable".to_string(), TokenType::VAR);
        keywords.insert("tantque".to_string(), TokenType::WHILE);
        keywords.insert("alors".to_string(), TokenType::THEN);
        keywords.insert("finsi".to_string(), TokenType::ENDIF);
        keywords.insert("faire".to_string(), TokenType::DO);
        keywords.insert("fintantque".to_string(), TokenType::ENDWHILE);
        keywords.insert("debut".to_string(), TokenType::START);
        keywords.insert("fin".to_string(), TokenType::END);

        Lexer {
            source: src,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            keywords,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> Option<char> {
        match self.source.chars().nth(self.current) {
            Some(c) => {
                self.current += 1;
                Some(c)
            }
            None => None,
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_literal(token_type, None);
    }

    fn add_token_literal(&mut self, token_type: TokenType, literal: Option<String>) {

        let text = match token_type{
            TokenType::STRING => {
                self.source[self.start+1..self.current-1].to_string()
            }
            _ => {
                self.source[self.start..self.current].to_string()
            }
        };
        let literal = match literal {
            Some(l) => l,
            None => text.clone(),
        };
        self.tokens
            .push(Token::new(token_type, text, literal, self.line));
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn peek(&self) -> Option<char> {
        match self.source.chars().nth(self.current) {
            Some(c) => Some(c),
            None => None,
        }
    }

    pub fn next_token(&mut self) -> TokenType {
        self.start = self.current;
        if self.is_at_end() {
            return TokenType::EOF;
        }

        let c = self.advance();
        match c {
            Some('(') => TokenType::LEFT_PAREN,
            Some(')') => TokenType::RIGHT_PAREN,
            Some('{') => TokenType::LEFT_BRACE,
            Some('}') => TokenType::RIGHT_BRACE,
            Some(',') => TokenType::COMMA,
            Some('.') => TokenType::DOT,
            Some('-') => TokenType::MINUS,
            Some('+') => TokenType::PLUS,
            Some(';') => TokenType::SEMICOLON,
            Some('*') => TokenType::STAR,
            Some('%') => TokenType::MODULO,
            Some(':') => TokenType::COLON,
            Some('[') => TokenType::LEFT_BRACKET,
            Some(']') => TokenType::RIGHT_BRACKET,
            Some('!') => {
                if self.match_next('=') {
                    TokenType::BANG_EQUAL
                } else {
                    TokenType::BANG
                }
            }
            Some('=') => {
                if self.match_next('=') {
                    TokenType::EQUAL_EQUAL
                } else {
                    TokenType::EQUAL
                }
            }
            Some('<') => {
              /*   if self.match_next('=') {
                    TokenType::LESS_EQUAL
                } else {
                    TokenType::LESS
                }*/

                match self.match_next('='){
                    true => TokenType::LESS_EQUAL,
                    false => {
                        match self.match_next('-'){
                            true => TokenType::ASSIGN,
                            false => TokenType::LESS
                        }
                    }
                }
            }
            Some('>') => {
                if self.match_next('=') {
                    TokenType::GREATER_EQUAL
                } else {
                    TokenType::GREATER
                }
            }
            Some('/') => {
                if self.match_next('/') {
                    while self.peek() != Some('\n') && !self.is_at_end() {
                        self.advance();
                    }
                    self.next_token()
                } else {
                    TokenType::SLASH
                }
            }
            Some(' ') | Some('\r') | Some('\t') => self.next_token(),
            Some('\n') => {
                self.line += 1;
                self.next_token()
            }
            Some('"') => self.string(),
            Some(c) => {
                if c.is_digit(10) {
                    self.number()
                } else if c.is_alphabetic() {
                    self.identifier()
                } else {
                   
                    self.next_token()
                }
            }
            None => TokenType::EOF,
        }
    }

    fn string(&mut self) -> TokenType {
        while self.peek() != Some('"') && !self.is_at_end() {
            if self.peek() == Some('\n') {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return TokenType::EOF;
        } else {
            self.advance();
        }

        TokenType::STRING
    }

    fn number(&mut self) -> TokenType {
        while let Some(c) = self.peek() {
            if c.is_digit(10) {
                self.advance();
            } else {
                break;
            }
        }

        if self.peek() == Some('.') && self.peek_next().unwrap().is_digit(10) {
            self.advance();
            while self.peek().unwrap().is_digit(10) {
                self.advance();
            }
        }

        TokenType::NUMBER
    }

    fn peek_next(&self) -> Option<char> {
        match self.source.chars().nth(self.current + 1) {
            Some(c) => Some(c),
            None => None,
        }
    }

    fn identifier(&mut self) -> TokenType {
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() {
                self.advance();
            } else {
                break;
            }
        }
    
        let text = &self.source[self.start..self.current];
        let token_type = match self.keywords.get(text) {
            Some(t) => (*t).clone(),
            None => TokenType::IDENTIFIER,
        };
    
        token_type
    }

    pub fn scan_tokens(&mut self) {
        loop {
            let token_type = self.next_token();

            if token_type == TokenType::EOF {
                break;
            }

            self.add_token(token_type);
        }
        self.add_token(TokenType::EOF);
    }
}
