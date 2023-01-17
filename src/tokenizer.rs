use anyhow::{anyhow, Context, Result};
use derive_more::Display;
use std::iter::Peekable;
use Token::*;

#[derive(Debug, Clone, PartialEq, Display)]
pub enum Token {
    Operator(char),
    Parenthesis(char),
    Bracket(char),
    Semicolon,
    Number(f64),
    Identifier(String),
    Imaginary,
}

#[derive(Default)]
pub struct Tokenizer;

impl<'a> Tokenizer {
    pub fn new() -> Self {
        Tokenizer {}
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
        Ok(identifier)
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

    pub fn tokenize(&self, input: &str) -> Result<Vec<Token>> {
        let mut lexer = input.chars().peekable();
        let mut tokenlist = Vec::new();
        while let Some(c) = lexer.next() {
            match c {
                '(' | ')' => tokenlist.push(Parenthesis(c)),
                '[' | ']' => tokenlist.push(Bracket(c)),
                ';' => tokenlist.push(Semicolon),
                '+' | '-' | '*' | '/' | '%' | '^' | '=' => tokenlist.push(Operator(c)),
                'i' if !is_alphabetical(lexer.peek().copied())  => tokenlist.push(Imaginary),
                'A'..='Z' | 'a'..='z' => tokenlist.push(Identifier(self.get_identifier(&mut lexer, c)?)),
                '0'..='9' => tokenlist.push(Number(self.get_number(&mut lexer, c)?)),
                c if c.is_whitespace() => {}
                _ => return Err(anyhow!("{}{}", "UNEXP_CHAR_ERR", c)),
            }
        }
        Ok(tokenlist)
    }
}

fn is_alphabetical(option: Option<char>) -> bool {
    if let Some(c) = option {
        c.is_alphabetic()
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_tokens(input: &str) -> Vec<Token> {
        let tokenizer = Tokenizer::new();
        tokenizer.tokenize(input).unwrap()
    }

    #[test]
    fn test_operators() {
        let mut tokens = get_tokens("+-*/%^=").into_iter();
        assert_eq!(Some(Token::Operator('+')), tokens.next());
        assert_eq!(Some(Token::Operator('-')), tokens.next());
        assert_eq!(Some(Token::Operator('*')), tokens.next());
        assert_eq!(Some(Token::Operator('/')), tokens.next());
        assert_eq!(Some(Token::Operator('%')), tokens.next());
        assert_eq!(Some(Token::Operator('^')), tokens.next());
        assert_eq!(Some(Token::Operator('=')), tokens.next());
        assert_eq!(None, tokens.next());
    }

    #[test]
    fn test_parenthesis_brackets_and_semicolon() {
        let mut tokens = get_tokens("()[];").into_iter();
        assert_eq!(Some(Token::Parenthesis('(')), tokens.next());
        assert_eq!(Some(Token::Parenthesis(')')), tokens.next());
        assert_eq!(Some(Token::Bracket('[')), tokens.next());
        assert_eq!(Some(Token::Bracket(']')), tokens.next());
        assert_eq!(Some(Token::Semicolon), tokens.next());
        assert_eq!(None, tokens.next());
    }

    #[test]
    fn test_numbers() {
        let mut tokens = get_tokens("0.0 42.0").into_iter();
        assert_eq!(Some(Token::Number(0.0)), tokens.next());
        assert_eq!(Some(Token::Number(42.0)), tokens.next());
        assert_eq!(None, tokens.next());
    }

    #[test]
    fn test_identifiers_and_imaginary() {
        let mut tokens = get_tokens("variable Function(x) i identifier i42 i").into_iter();
        assert_eq!(Some(Token::Identifier("variable".to_owned())), tokens.next());
        assert_eq!(Some(Token::Identifier("Function".to_owned())), tokens.next());
        assert_eq!(Some(Token::Parenthesis('(')), tokens.next());
        assert_eq!(Some(Token::Identifier("x".to_owned())), tokens.next());
        assert_eq!(Some(Token::Parenthesis(')')), tokens.next());
        assert_eq!(Some(Token::Imaginary), tokens.next());
        assert_eq!(Some(Token::Identifier("identifier".to_owned())), tokens.next());
        assert_eq!(Some(Token::Imaginary), tokens.next());
        assert_eq!(Some(Token::Number(42.0)), tokens.next());
        assert_eq!(Some(Token::Imaginary), tokens.next());
        assert_eq!(None, tokens.next());
    }
}