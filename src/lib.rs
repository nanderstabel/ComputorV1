use anyhow::{anyhow, Context, Result};
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;
use std::{iter::Peekable, ops::Deref, vec::IntoIter};
use Token::*;

macro_rules! node {
    ($token:expr) => {
        node!($token, None, None)
    };
    ($token:expr, $left:expr) => {
        node!($token, $left, None)
    };
    ($token:expr, $left:expr, $right:expr) => {
        Some(Rc::new(RefCell::new(Node::new($token, $left, $right))))
    };
}

pub type Branch = Option<Rc<RefCell<Node>>>;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Token {
    Operator(char),
    Parenthesis(char),
    Number(f64),
    Identifier(char),
}

#[derive(Debug, Clone)]
pub struct Node {
    token: Rc<RefCell<Token>>,
    left: Branch,
    right: Branch,
}

impl Node {
    pub fn new(token: Token, left: Branch, right: Branch) -> Node {
        Node {
            token: Rc::new(RefCell::new(token)),
            left,
            right,
        }
    }

    pub fn left(&self) -> Branch {
        self.left.clone()
    }

    pub fn right(&self) -> Branch {
        self.right.clone()
    }
}

use itertools::Itertools;

impl IntoIterator for Node {
    type Item = Rc<RefCell<Token>>;

    type IntoIter = NodeIter;

    fn into_iter(self) -> Self::IntoIter {
        let left_iter = self
            .left()
            .map(|left| Rc::new(RefCell::new(left.as_ref().borrow_mut().clone().into_iter())));
        let right_iter = self.right().map(|right| {
            Rc::new(RefCell::new(
                right.as_ref().borrow_mut().clone().into_iter(),
            ))
        });

        NodeIter {
            node: self,
            count: 0,
            left_iter,
            right_iter,
        }
    }
}

pub struct NodeIter {
    node: Node,
    pub count: usize,
    left_iter: Option<Rc<RefCell<NodeIter>>>,
    right_iter: Option<Rc<RefCell<NodeIter>>>,
}

impl Iterator for NodeIter {
    type Item = Rc<RefCell<Token>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.count {
            0 => {
                self.count += 1;
                Some(self.node.token.clone())
            }
            1 => self.left_iter.as_ref().and_then(|left_iter| {
                left_iter.as_ref().borrow_mut().next().or_else(|| {
                    self.right_iter
                        .as_ref()
                        .map(|right_iter| right_iter.as_ref().borrow_mut().next())
                        .flatten()
                })
            }),
            _ => None,
        }
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
