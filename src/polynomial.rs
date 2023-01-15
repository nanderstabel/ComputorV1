use crate::node::Branch;
use crate::tokenizer::Token::*;
use derive_more::{Deref, DerefMut};
use itertools::Itertools;
use merge::Merge;
use std::{
    cmp::Ordering::*,
    fmt::{write, Display},
    ops::Add,
};

#[derive(Debug, Default, Merge, PartialEq, Clone)]
pub struct Term {
    #[merge(skip)]
    pub is_sign_negative: bool,
    pub coefficient: Option<f64>,
    pub operator: Option<char>,
    pub identifier: Option<String>,
    pub exponent: Option<f64>,
}

impl Add for Term {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let coefficient = match (self.is_sign_negative, rhs.is_sign_negative) {
            (true, true) => -self.coefficient.unwrap_or(1.0) - rhs.coefficient.unwrap_or(1.0),
            (true, false) => -self.coefficient.unwrap_or(1.0) + rhs.coefficient.unwrap_or(1.0),
            (false, true) => self.coefficient.unwrap_or(1.0) - rhs.coefficient.unwrap_or(1.0),
            (false, false) => self.coefficient.unwrap_or(1.0) + rhs.coefficient.unwrap_or(1.0),
        };
        Term {
            is_sign_negative: coefficient.is_sign_negative(),
            coefficient: Some(coefficient.abs()),
            operator: self.operator,
            identifier: self.identifier,
            exponent: self.exponent,
        }
    }
}

impl PartialOrd for Term {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.exponent
            .partial_cmp(&other.exponent)
            .and_then(|ord| match ord {
                Equal => {
                    self.coefficient
                        .partial_cmp(&other.coefficient)
                        .and_then(|ord| match ord {
                            Equal => self.identifier.partial_cmp(&other.identifier),
                            _ => Some(ord),
                        })
                }
                _ => Some(ord),
            })
    }
}

impl From<Branch> for Term {
    fn from(branch: Branch) -> Self {
        let node = branch.borrow().clone();
        let mut term = Term::default();
        match node.token {
            Operator('-') | Operator('+') => panic!(),
            Operator(operator) => {
                term.operator = Some(operator);
                match operator {
                    '^' => {
                        term.merge(Term::from(node.left.unwrap()));
                        if let Some(right) = node.right {
                            let right_node = right.borrow().clone();
                            match right_node.token {
                                Number(exponent) => term.exponent = Some(exponent),
                                _ => unimplemented!(),
                            }
                        }
                    }
                    '*' | '/' | '%' => {
                        term.merge(Term::from(node.left.unwrap()));
                        term.merge(Term::from(node.right.unwrap()));
                    }
                    _ => panic!(),
                }
            }
            Identifier(identifier) => term.identifier = Some(identifier),
            Number(coefficient) => term.coefficient = Some(coefficient),
            _ => unreachable!(),
        }
        term
    }
}

impl Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_sign_negative {
            write!(f, "- ")?;
        }
        if let Some(coefficient) = self.coefficient {
            write!(f, "{coefficient}")?;
        }

        if let Some(operator) = self.operator {
            write!(f, " {operator} ")?;
        }

        if let Some(identifier) = &self.identifier {
            write!(f, "{identifier}")?;
        }
        if let Some(exponent) = self.exponent {
            write!(f, "^{exponent}")?;
        }
        Ok(())
    }
}

#[derive(Debug, Default, DerefMut, Deref)]
pub struct Polynomial(pub Vec<Term>);

impl Polynomial {
    pub fn reduce(&mut self) {
        self.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mut reduced_terms = vec![];
        for (a, b) in self.iter().tuple_windows() {
            match (
                a.identifier.partial_cmp(&b.identifier),
                a.exponent.partial_cmp(&b.exponent),
            ) {
                (Some(Equal), Some(Equal)) => reduced_terms.push(a.clone() + b.clone()),
                _ => reduced_terms.push(b.clone()),
            }
        }
        self.0 = reduced_terms;
    }
}

impl From<Branch> for Polynomial {
    fn from(branch: Branch) -> Self {
        let node = branch.borrow().clone();

        let mut polynomial = Polynomial::default();
        match node.token {
            Operator('-') | Operator('+') => {
                polynomial.append(&mut Polynomial::from(node.left.unwrap()));
                let mut right = Polynomial::from(node.right.unwrap());
                if let Some(coefficient) = right[0].coefficient.replace(-1.0) {
                    right[0].coefficient = Some(-coefficient)
                }
                polynomial.append(&mut Polynomial::from(right));
            }
            _ => polynomial.push(Term::from(branch.clone())),
        }
        polynomial
    }
}

impl Display for Polynomial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for term in self.iter() {
            write!(f, " {term}")?;
        }
        Ok(())
    }
}
