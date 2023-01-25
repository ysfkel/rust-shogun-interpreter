use std::fmt::format;

use super::super::token::token::{self, Token};

pub struct Lexer {
    input: String,
    position: u64,      // current position in input (points to current char)
    read_position: u64, // current reading position in input (after current char)
    ch: char,           // current char under examination
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut l = Self {
            input,
            position: 0,
            read_position: 0,
            ch: ' ',
        };
        l.read_char();
        l
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let tok = match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    let literal = format!("{}{}", ch, self.ch.to_string());
                    Token::new(token::EQ, &literal)
                } else {
                    Token::new(token::ASSIGN, &self.ch.to_string())
                }
            }

            '!' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    let literal = format!("{}{}", ch, self.ch.to_string());
                    Token::new(token::NOT_EQ, &literal)
                } else {
                    Token::new(token::BANG, &self.ch.to_string())
                }
            }
            '<' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    let literal = format!("{}{}", ch, self.ch.to_string());
                    Token::new(token::LT_EQ, &literal)
                } else {
                    Token::new(token::LT, &self.ch.to_string())
                }
            }
            '>' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    let literal = format!("{}{}", ch, self.ch.to_string());
                    Token::new(token::GT_EQ, &literal)
                } else {
                    Token::new(token::GT, &self.ch.to_string())
                }
            }
            '|' => {
                if self.peek_char() == '|' {
                    let ch = self.ch;
                    self.read_char();
                    let literal = format!("{}{}", ch, self.ch.to_string());
                    Token::new(token::OR, &literal)
                } else {
                    Token::new(token::B_OR, &self.ch.to_string())
                }
            }

            '&' => {
                if self.peek_char() == '&' {
                    let ch = self.ch;
                    self.read_char();
                    let literal = format!("{}{}", ch, self.ch.to_string());
                    Token::new(token::AND, &literal)
                } else {
                    Token::new(token::B_AND, &self.ch.to_string())
                }
            }
            '+' => Token::new(token::PLUS, &self.ch.to_string()),
            '-' => Token::new(token::MINUS, &self.ch.to_string()),
            '/' => Token::new(token::SLASH, &self.ch.to_string()),
            '*' => Token::new(token::ASTERISK, &self.ch.to_string()),
            ';' => Token::new(token::SEMICOLON, &self.ch.to_string()),
            ',' => Token::new(token::COMMA, &self.ch.to_string()),
            '{' => Token::new(token::LBRACE, &self.ch.to_string()),
            '}' => Token::new(token::RBRACE, &self.ch.to_string()),
            '(' => Token::new(token::LPAREN, &self.ch.to_string()),
            ')' => Token::new(token::RPAREN, &self.ch.to_string()),
            '\0' => Token::new(token::EOF, &""),
            _ => {
                if self.is_letter(self.ch) {
                    let literal = self.read_dentifier();

                    return Token::new(&Token::lookup_indent(&literal), &literal);
                } else if self.is_digit(self.ch) {
                    return Token::new(token::INT, &self.read_number());
                } else {
                    return Token::new(token::ILLEGAL, &self.ch.to_string());
                }
            }
        };
        self.read_char();
        tok
    }

    fn read_char(&mut self) {
        if self.is_invalid_read_position() {
            self.ch = '\0';
        } else {
            self.ch = self.read_input();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&self) -> char {
        if self.is_invalid_read_position() {
            '\0'
        } else {
            self.read_input()
        }
    }

    fn read_input(&self) -> char {
        self.input.chars().nth(self.read_position as usize).unwrap()
    }

    fn is_invalid_read_position(&self) -> bool {
        let input_len: u64 = self.input.len().try_into().unwrap();
        self.read_position >= input_len
    }

    fn read_dentifier(&mut self) -> String {
        let position = self.position;

        while self.is_identifier_character(self.ch) {
            self.read_char();
        }
        let ch: &str = &self.input[position as usize..self.position as usize];
        ch.into()
    }

    fn is_letter(&self, ch: char) -> bool {
        'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
    }

    fn is_digit(&self, ch: char) -> bool {
        '0' <= ch && ch <= '9'
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while self.is_digit(self.ch) {
            self.read_char();
        }

        let ch: &str = &self.input[position as usize..self.position as usize];
        ch.into()
    }

    fn is_identifier_character(&self, ch: char) -> bool {
        self.is_letter(ch) || ch.is_digit(10)
    }

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }
}
