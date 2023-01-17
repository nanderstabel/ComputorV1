pub mod polynomial;
pub mod matrix;
pub mod complex;
pub mod rational;
pub mod function;
pub mod variable;

use std::{fmt::{Debug, Display}, rc::Rc};

use polynomial::Term;

use crate::node::NodeObject;

pub trait Type: Debug + Display {
    fn node_color<'a>(&self) -> &'a str {
        "#FFFFFF"
    }

    fn into_term(&self) -> Term;

    fn into_node_object(self) -> NodeObject where Self: Sized + 'static {
        NodeObject::Operand(Rc::new(self))
    }
}