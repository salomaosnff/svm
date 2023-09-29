use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Location {
  pub column: usize,
  pub row: usize,
}

#[derive(Debug, PartialEq)]
pub enum Token {
  Keyword(String, Location),
  IdentifierName(String, Location),
  Punctuator(String, Location),
  NumericLiteral(String, Location),
  StringLiteral(String, Location),

  WhiteSpace(String, Location),
  Reserved(String, Location),
  Invalid(char, Location),
  End(Location),
}

impl Display for Location {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "({}:{})", self.row, self.column)
  }
}
