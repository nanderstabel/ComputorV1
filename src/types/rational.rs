use super::{Type, polynomial::Term};

#[derive(Debug)]
pub struct Rational(pub f64);

impl Type for Rational {
    fn into_term(&self) -> Term {
        let mut term = Term::default();
        
        if self.0.is_sign_negative() {
            term.is_sign_negative = true;
            term.coefficient = Some(-self.0);
        } else {
            term.coefficient = Some(self.0);
        }
        term
    }
}