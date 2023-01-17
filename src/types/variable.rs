use super::{Type, polynomial::Term};

#[derive(Debug)]
pub struct Variable(pub String);

impl Type for Variable {
    fn into_term(&self) -> Term {
        let mut term = Term::default();

        term.identifier = Some(self.0.clone());
        term
    }
}