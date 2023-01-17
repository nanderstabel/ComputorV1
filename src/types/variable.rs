use super::Type;

#[derive(Debug)]
pub struct Variable(pub String);

impl Type for Variable {}