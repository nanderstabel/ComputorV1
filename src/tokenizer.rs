use anyhow::{anyhow, Context, Result};
use std::iter::Peekable;
use Token::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Operator(char),
    Parenthesis(char),
    Number(f64),
    Identifier(String),
}

#[derive(Default)]
pub struct Tokenizer;

impl<'a> Tokenizer {
    pub fn new() -> Self {
        Tokenizer {}
    }

    fn get_number<I>(&self, lexer: &mut Peekable<I>, c: char) -> Result<f64>
    where
        I: Iterator<Item = char>,
    {
        let mut number = c.to_string();
        let mut is_float = false;
        while let Some(c) = lexer.peek() {
            match c {
                '0'..='9' => number.push(*c),
                '.' if !is_float => {
                    number.push(*c);
                    is_float = true;
                }
                _ => break,
            }
            lexer.next();
        }
        number.parse().context("PARSE_ERROR")
    }

    fn get_identifier<I>(&self, lexer: &mut Peekable<I>, c: char) -> Result<String>
    where
        I: Iterator<Item = char>,
    {
        let mut identifier = c.to_string();
        while let Some(c) = lexer.peek() {
            match c {
                '0'..='9' | 'a'..='z' | 'A'..='Z' => identifier.push(*c),
                _ => break,
            }
            lexer.next();
        }
        Ok(identifier.to_lowercase())
    }

    pub fn tokenize(&self, input: &str) -> Result<Vec<Token>> {
        let mut lexer = input.chars().peekable();
        let mut tokenlist: Vec<Token> = Vec::new();
        while let Some(c) = lexer.next() {
            match c {
                '(' | ')' => tokenlist.push(Parenthesis(c)),
                '+' | '-' | '*' | '/' | '%' | '^' | '=' => tokenlist.push(Operator(c)),
                'A'..='Z' => tokenlist.push(Identifier(self.get_identifier(&mut lexer, c)?)),
                '0'..='9' => tokenlist.push(Number(self.get_number(&mut lexer, c)?)),
                c if c.is_whitespace() => {}
                _ => return Err(anyhow!("{}{}", "UNEXP_CHAR_ERR", c)),
            }
        }
        Ok(tokenlist)
    }
}
