use derive_more::Display;
use super::{Type, polynomial::Term};

#[derive(Debug, Display, Clone)]
#[display(fmt = "{}({})", identifier, arg)]
pub struct Function {
    pub identifier: String,
    pub arg: String
}

impl Type for Function {
    // fn node_color<'a>(&self) -> &'a str {
    //     "#00A0B0"
    // }

    fn into_term(&self) -> Term {
        let mut term = Term::default();
        term.identifier = Some(self.identifier.clone());
        term
    }
}