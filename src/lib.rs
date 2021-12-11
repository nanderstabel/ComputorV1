use anyhow::{anyhow, Context, Result};
use std::iter::Peekable;
use Token::*;

macro_rules! node {
    ($token:expr) => {
        node!($token, Box::new(None), Box::new(None))
    };
    ($token:expr, $left:expr) => {
        node!($token, $left, Box::new(None))
    };
    ($token:expr, $left:expr, $right:expr) => {
        Box::new(Some(Node::new($token, $left, $right)))
    };
}

pub type Branch = Box<Option<Node>>;

#[derive(Debug, Copy, Clone)]
pub enum Token {
    Operator(char),
    Parenthesis(char),
    Number(f64),
    Identifier(char),
}

#[derive(Debug)]
pub struct Node {
    token: Token,
    left: Branch,
    right: Branch,
}

impl Node {
    pub fn new(token: Token, left: Branch, right: Branch) -> Node {
        Node { token, left, right }
    }
}

pub struct Parser;

impl<'a> Parser {
    pub fn new() -> Self {
        Parser {}
    }

    fn get_number<I>(&mut self, lexer: &mut Peekable<I>, c: char) -> Result<f64>
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

    fn tokenize(&mut self, input: &str) -> Result<Vec<Token>> {
        let mut lexer = input.chars().peekable();
        let mut tokenlist: Vec<Token> = Vec::new();
        while let Some(c) = lexer.next() {
            match c {
                '(' | ')' => tokenlist.push(Parenthesis(c)),
                '+' | '-' | '*' | '/' | '^' | '=' => tokenlist.push(Operator(c)),
                'A'..='Z' => tokenlist.push(Identifier(c)),
                '0'..='9' => tokenlist.push(Number(self.get_number(&mut lexer, c)?)),
                //TODO:further implement identifiers
                c if c.is_whitespace() => {}
                _ => return Err(anyhow!("{}{}", "UNEXP_CHAR_ERR", c)),
            }
        }
        Ok(tokenlist)
    }

    fn equation<I>(&mut self, tokenlist: &mut Peekable<I>) -> Result<Branch>
    where
        I: Iterator<Item = &'a Token>,
    {
        let lhs = self.expression(tokenlist);
        match tokenlist.next() {
            Some(Operator('=')) => {
                let rhs = self.expression(tokenlist);
                match tokenlist.next() {
                    None => Ok(node!(Operator('-'), lhs?, rhs?)),
                    Some(t) => Err(anyhow!("{}{:?}", "UNEXP_TOKEN_ERR", t)),
                }
            }
            _ => Err(anyhow!("MISSING_IMPLICATOR_ERR")),
        }
    }

    fn expression<I>(&mut self, tokenlist: &mut Peekable<I>) -> Result<Branch>
    where
        I: Iterator<Item = &'a Token>,
    {
        let mut node = self.term(tokenlist);
        while let Some(Operator('+')) | Some(Operator('-')) = tokenlist.peek() {
            node = Ok(node!(
                *tokenlist.next().context("UNEXP_END_ERR")?,
                node?,
                self.term(tokenlist)?
            ));
        }
        node
    }

    fn term<I>(&mut self, tokenlist: &mut Peekable<I>) -> Result<Branch>
    where
        I: Iterator<Item = &'a Token>,
    {
        let mut node = self.factor(tokenlist);
        while let Some(Operator('*')) | Some(Operator('/')) = tokenlist.peek() {
            node = Ok(node!(
                *tokenlist.next().context("UNEXP_END_ERR")?,
                node?,
                self.factor(tokenlist)?
            ));
        }
        node
    }

    fn factor<I>(&mut self, tokens: &mut Peekable<I>) -> Result<Branch>
    where
        I: Iterator<Item = &'a Token>,
    {
        let lhs = self.primary(tokens);
        match tokens.peek() {
            Some(Operator('^')) => {
                let parent = tokens.next().context("INSERT ERROR")?;
                let rhs = self.factor(tokens);
                Ok(node!(*parent, lhs?, rhs?))
            }
            _ => lhs,
        }
    }

    fn primary<I>(&mut self, tokenlist: &mut Peekable<I>) -> Result<Branch>
    where
        I: Iterator<Item = &'a Token>,
    {
        let token = tokenlist.next();
        match token {
            Some(Parenthesis('(')) => {
                let node = self.expression(tokenlist);
                match tokenlist.next() {
                    Some(Parenthesis(')')) => node,
                    _ => Err(anyhow!("MISSING_PAREN_ERR")),
                }
            }
            Some(Operator('!')) => Ok(node!(
                *token.context("UNEXP_END_ERR")?,
                self.primary(tokenlist)?
            )),
            Some(Identifier(_)) => Ok(node!(*token.context("UNEXP_END_ERR")?)),
            Some(Number(_)) => Ok(node!(*token.context("UNEXP_END_ERR")?)),
            _ => Err(anyhow!("UNEXP_END_ERR")),
        }
    }

    pub fn parse(&mut self, input: &str) -> Result<Branch> {
        let tokenlist = self.tokenize(input).context("TOKENIZATION_ERR")?;
        println!("{:?}", tokenlist);
        let tree = self
            .equation(&mut tokenlist.iter().peekable())
            .context("SYNTAX_ERR")?;
        Ok(tree)
    }
}

fn get_identifier<I>(lexer: &mut Peekable<I>, c: char) -> String
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
    identifier
}
