use derive_more::Display;
use super::{Type, polynomial::Term};

#[derive(Debug, Display)]
pub struct Variable(pub String);

impl Type for Variable {
    fn node_color<'a>(&self) -> &'a str {
        "#D3643B"
    }

    fn into_term(&self) -> Term {
        let mut term = Term::default();

        term.identifier = Some(self.0.clone());
        term
    }
}