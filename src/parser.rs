use crate::node::{Branch, Node};
use crate::tokenizer::{Token, Token::*, Tokenizer};
use crate::types::rational::Rational;
use crate::types::variable::Variable;
use anyhow::{anyhow, Context, Result};
use std::iter::Peekable;
use std::rc::Rc;
use crate::node::NodeObject;

#[derive(Default)]
pub struct Parser;

impl<'a> Parser {
    pub fn new() -> Self {
        Parser {}
    }

    fn equation<I>(&self, tokenlist: &mut Peekable<I>) -> Result<Option<Branch>>
    where
        I: Iterator<Item = &'a Token>,
    {
        let lhs = self.expression(tokenlist);
        match tokenlist.next() {
            Some(Operator('=')) => {
                let rhs = self.expression(tokenlist);
                match tokenlist.next() {
                    None => Ok(node!((&Operator('-')).into(), lhs?, rhs?)),
                    Some(t) => Err(anyhow!("{}{:?}", "UNEXP_TOKEN_ERR", t)),
                }
            }
            _ => Err(anyhow!("MISSING_IMPLICATOR_ERR")),
        }
    }

    fn expression<I>(&self, tokenlist: &mut Peekable<I>) -> Result<Option<Branch>>
    where
        I: Iterator<Item = &'a Token>,
    {
        let mut node = self.term(tokenlist);
        while let Some(Operator('+')) | Some(Operator('-')) = tokenlist.peek() {
            node = Ok(node!(
                tokenlist.next().context("UNEXP_END_ERR")?.into(),
                node?,
                self.term(tokenlist)?
            ));
        }
        node
    }

    fn term<I>(&self, tokenlist: &mut Peekable<I>) -> Result<Option<Branch>>
    where
        I: Iterator<Item = &'a Token>,
    {
        let mut node = self.factor(tokenlist);
        while let Some(Operator('*')) | Some(Operator('/')) | Some(Operator('%')) = tokenlist.peek()
        {
            node = Ok(node!(
                tokenlist.next().context("UNEXP_END_ERR")?.into(),
                node?,
                self.factor(tokenlist)?
            ));
        }
        node
    }

    fn factor<I>(&self, tokens: &mut Peekable<I>) -> Result<Option<Branch>>
    where
        I: Iterator<Item = &'a Token>,
    {
        let lhs = self.primary(tokens);
        match tokens.peek() {
            Some(Operator('^')) => {
                let parent = tokens.next().context("INSERT ERROR")?;
                // Keep this: let rhs = self.factor(tokens);
                // This is a temporary solution in order to cast the exponent as a Token rather than a Rational.
                let temp = if let Some(Number(number)) = tokens.next() {
                    node!((&Number(*number)).into(), None, None)
                } else {
                    panic!()
                };
                Ok(node!(parent.into(), lhs?, temp))
            }
            _ => lhs,
        }
    }

    fn primary<I>(&self, tokenlist: &mut Peekable<I>) -> Result<Option<Branch>>
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
            Some(Identifier(identifier)) => Ok(node!(NodeObject::Operand(Rc::new(Variable(identifier.clone()))))),
            Some(Number(number)) => Ok(node!(NodeObject::Operand(Rc::new(Rational(*number))))),
            _ => Err(anyhow!("UNEXP_END_ERR")),
        }
    }

    pub fn parse(&self, input: &str) -> Result<Option<Branch>> {
        let tokenizer = Tokenizer::new();
        let tokenlist = tokenizer.tokenize(input).context("TOKENIZATION_ERR")?;
        self.equation(&mut tokenlist.iter().peekable())
            .context("SYNTAX_ERR")
    }
}
