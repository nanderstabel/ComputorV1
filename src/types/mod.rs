pub mod polynomial;
pub mod matrix;
pub mod complex;
pub mod rational;
pub mod function;
pub mod variable;

use std::fmt::{Debug, Display};

use polynomial::Term;

pub trait Type: Debug + Display {
    fn node_color<'a>(&self) -> &'a str {
        "#FFFFFF"
    }

    fn into_term(&self) -> Term;
}