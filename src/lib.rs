use anyhow::{anyhow, Context, Result};
use std::cell::RefCell;
use std::iter::Peekable;
use std::rc::Rc;
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

pub type Branch = Rc<RefCell<Node>>;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Operator(char),
    Parenthesis(char),
    Number(f64),
    // Identifier(char),
    Identifier(String),
}

#[derive(Debug, Clone)]
pub struct Node {
    pub token: Token,
    left: Option<Branch>,
    right: Option<Branch>,
}

impl Node {
    pub fn new(token: Token, left: Option<Branch>, right: Option<Branch>) -> Node {
        Node { token, left, right }
    }
}

impl IntoIterator for Node {
    type Item = Rc<RefCell<Self>>;
    type IntoIter = NodeIter;

    fn into_iter(self) -> Self::IntoIter {
        let left = self.left.clone();
        let right = self.right.clone();

        NodeIter {
            node: Some(Rc::new(RefCell::new(self))),
            children: if let Some(left) = left {
                let iter = left.borrow_mut().clone().into_iter();
                if let Some(right) = right {
                    Some(Rc::new(RefCell::new(
                        iter.chain(right.borrow_mut().clone().into_iter()),
                    )))
                } else {
                    Some(Rc::new(RefCell::new(iter)))
                }
            } else {
                None
            },
        }
    }
}

pub struct NodeIter {
    node: Option<Branch>,
    children: Option<Rc<RefCell<dyn Iterator<Item = Branch>>>>,
}

impl Iterator for NodeIter {
    type Item = Branch;

    fn next(&mut self) -> Option<Self::Item> {
        self.node.take().or_else(|| {
            self.children
                .as_ref()
                .and_then(|iter| iter.borrow_mut().next())
        })
    }
}

#[derive(Default)]
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

    fn get_identifier<I>(&mut self, lexer: &mut Peekable<I>, c: char) -> Result<String>
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

    fn tokenize(&mut self, input: &str) -> Result<Vec<Token>> {
        let mut lexer = input.chars().peekable();
        let mut tokenlist: Vec<Token> = Vec::new();
        while let Some(c) = lexer.next() {
            match c {
                '(' | ')' => tokenlist.push(Parenthesis(c)),
                '+' | '-' | '*' | '/' | '^' | '=' => tokenlist.push(Operator(c)),
                'A'..='Z' => tokenlist.push(Identifier(self.get_identifier(&mut lexer, c)?)),
                '0'..='9' => tokenlist.push(Number(self.get_number(&mut lexer, c)?)),
                c if c.is_whitespace() => {}
                _ => return Err(anyhow!("{}{}", "UNEXP_CHAR_ERR", c)),
            }
        }
        Ok(tokenlist)
    }

    fn equation<I>(&mut self, tokenlist: &mut Peekable<I>) -> Result<Option<Branch>>
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

    fn expression<I>(&mut self, tokenlist: &mut Peekable<I>) -> Result<Option<Branch>>
    where
        I: Iterator<Item = &'a Token>,
    {
        let mut node = self.term(tokenlist);
        while let Some(Operator('+')) | Some(Operator('-')) = tokenlist.peek() {
            node = Ok(node!(
                tokenlist.next().context("UNEXP_END_ERR")?.clone(),
                node?,
                self.term(tokenlist)?
            ));
        }
        node
    }

    fn term<I>(&mut self, tokenlist: &mut Peekable<I>) -> Result<Option<Branch>>
    where
        I: Iterator<Item = &'a Token>,
    {
        let mut node = self.factor(tokenlist);
        while let Some(Operator('*')) | Some(Operator('/')) = tokenlist.peek() {
            node = Ok(node!(
                tokenlist.next().context("UNEXP_END_ERR")?.clone(),
                node?,
                self.factor(tokenlist)?
            ));
        }
        node
    }

    fn factor<I>(&mut self, tokens: &mut Peekable<I>) -> Result<Option<Branch>>
    where
        I: Iterator<Item = &'a Token>,
    {
        let lhs = self.primary(tokens);
        match tokens.peek() {
            Some(Operator('^')) => {
                let parent = tokens.next().context("INSERT ERROR")?;
                let rhs = self.factor(tokens);
                Ok(node!(parent.clone(), lhs?, rhs?))
            }
            _ => lhs,
        }
    }

    fn primary<I>(&mut self, tokenlist: &mut Peekable<I>) -> Result<Option<Branch>>
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
                token.context("UNEXP_END_ERR")?.clone(),
                self.primary(tokenlist)?
            )),
            Some(Identifier(_)) => Ok(node!(token.context("UNEXP_END_ERR")?.clone())),
            Some(Number(_)) => Ok(node!(token.context("UNEXP_END_ERR")?.clone())),
            _ => Err(anyhow!("UNEXP_END_ERR")),
        }
    }

    pub fn parse(&mut self, input: &str) -> Result<Option<Branch>> {
        let tokenlist = self.tokenize(input).context("TOKENIZATION_ERR")?;
        println!("{:?}", tokenlist);
        let tree = self
            .equation(&mut tokenlist.iter().peekable())
            .context("SYNTAX_ERR")?;
        Ok(tree)
    }
}
