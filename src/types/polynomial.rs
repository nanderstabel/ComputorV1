use crate::node::Branch;
use crate::tokenizer::Token::*;
use derive_more::{Deref, DerefMut};
use itertools::Itertools;
use merge::Merge;
use std::{cmp::Ordering::*, fmt::Display, ops::Add};

#[derive(Debug, Default, Merge, PartialEq, Clone)]
pub struct Term {
    #[merge(skip)]
    pub is_sign_negative: bool,
    pub coefficient: Option<f64>,
    pub operator: Option<char>,
    pub identifier: Option<String>,
    pub exponent: Option<f64>,
}

impl Term {
    pub fn coefficient(&self) -> f64 {
        if self.is_sign_negative {
            -self.coefficient.unwrap()
        } else {
            self.coefficient.unwrap()
        }
    }
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
        let operator = if self.operator.is_none() && rhs.operator.is_none() {
            Some('*')
        } else {
            self.operator
        };
        Term {
            is_sign_negative: coefficient.is_sign_negative(),
            coefficient: Some(coefficient.abs()),
            // operator: self.operator,
            operator,
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
                Equal => self.identifier.partial_cmp(&other.identifier),
                // .and_then(|ord| match ord {
                //     Equal => self.coefficient.partial_cmp(&other.coefficient),
                //     _ => Some(ord),
                // }),
                _ => Some(ord),
            })
    }
}

impl From<Branch> for Term {
    fn from(branch: Branch) -> Self {
        let node = branch.borrow().clone();
        let mut term = Term::default();

        match node.object.into() {
            Operator('-') | Operator('+') => panic!(),
            Operator(operator) => match operator {
                '^' => {
                    term.merge(Term::from(node.left.unwrap()));
                    if let Some(right) = node.right {
                        let right_node = right.borrow().clone();
                        match right_node.object.into() {
                            Number(exponent) => term.exponent = Some(exponent),
                            _ => unimplemented!(),
                        }
                    }
                }
                '*' | '/' | '%' => {
                    term.operator = Some(operator);
                    term.merge(Term::from(node.left.unwrap()));
                    term.merge(Term::from(node.right.unwrap()));
                }
                _ => panic!(),
            },
            Identifier(identifier) => term.identifier = Some(identifier),
            Number(coefficient) => {
                if coefficient.is_sign_negative() {
                    term.is_sign_negative = true;
                    term.coefficient = Some(-coefficient);
                } else {
                    term.coefficient = Some(coefficient);
                }
            }
            _ => unreachable!(),
        }
        term
    }
}

impl Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(coefficient) = self.coefficient {
            write!(f, "{coefficient} ")?;
        }
        if let Some(operator) = self.operator {
            write!(f, "{operator} ")?;
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
pub struct Polynomial(Vec<Term>);

impl Polynomial {
    pub fn reduce(&mut self) {
        self.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mut dummy = Term::default();
        dummy.exponent = Some(f64::MAX);
        self.0.push(dummy);

        let mut pairs = self.iter().tuple_windows::<(&Term, &Term)>();
        let mut reduced_terms = vec![];
        while let Some((a, b)) = pairs.next() {
            match (
                a.is_sign_negative ^ b.is_sign_negative,
                a.coefficient.partial_cmp(&b.coefficient),
                a.operator.partial_cmp(&b.operator),
                a.identifier.partial_cmp(&b.identifier),
                a.exponent.partial_cmp(&b.exponent),
            ) {
                (true, Some(Equal), Some(Equal), Some(Equal), Some(Equal)) => {
                    pairs.next();
                }
                (_, _, _, Some(Equal), Some(Equal)) => {
                    reduced_terms.push(a.clone() + b.clone());
                    pairs.next();
                }
                _ => reduced_terms.push(a.clone()),
            }
        }
        self.0 = reduced_terms;
    }

    pub fn degree(&mut self) -> usize {
        self.iter().fold(0, |degree, current| {
            degree.max(current.exponent.unwrap() as usize)
        })
    }

    pub fn solve(&mut self) -> Vec<f64> {
        let mut terms = self.0.clone();
        terms.reverse();

        let degree = self.degree();

        let mut coefficients = terms.iter().map(|t| t.coefficient());
        let a = coefficients.next().unwrap_or_default();
        let b = coefficients.next().unwrap_or_default();
        let c = coefficients.next().unwrap_or_default();
        let _d = coefficients.next().unwrap_or_default();

        match degree {
            1 => {
                vec![-(b / a)]
            }
            2 => match (b * b - 4. * a * c).partial_cmp(&0.0) {
                Some(Greater) => vec![
                    (-b - f64::sqrt(b * b - 4. * a * c)) / (2.0 * a),
                    (-b + f64::sqrt(b * b - 4. * a * c)) / (2.0 * a),
                ],
                Some(Equal) => vec![-b / (2. * a)],
                Some(Less) => vec![],
                None => panic!(),
            },
            3 => {
                todo!();
            }
            _ => unimplemented!(),
        }
    }
}

impl From<Branch> for Polynomial {
    fn from(branch: Branch) -> Self {
        let node = branch.borrow().clone();

        let mut polynomial = Polynomial::default();
        match node.object.into() {
            Operator(operator) if operator == '+' || operator == '-' => {
                polynomial.append(&mut Polynomial::from(node.left.unwrap()));
                let mut right = Polynomial::from(node.right.unwrap());
                if operator == '-' {
                    right[0].is_sign_negative = true;
                    // if let Some(coefficient) = right[0].coefficient.replace(1.0) {
                    //     right[0].coefficient = Some(-coefficient)
                    // }
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
        let mut terms = self.iter();
        if let Some(first) = terms.next() {
            if first.is_sign_negative {
                write!(f, "- ")?;
            }
            write!(f, "{first} ")?;
            for term in terms {
                write!(f, "{} ", if term.is_sign_negative { '-' } else { '+' })?;
                write!(f, "{term} ")?;
            }
        }
        Ok(())
    }
}
