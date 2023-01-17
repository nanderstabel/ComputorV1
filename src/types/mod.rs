pub mod polynomial;
pub mod matrix;
pub mod complex;
pub mod rational;
pub mod function;
pub mod variable;

use std::{fmt::Debug};
use polynomial::Term;

pub trait Type: Debug {
    fn node_color<'a>(&self) -> &'a str {
        "#000000"
    }

    fn into_term(&self) -> Term;
}