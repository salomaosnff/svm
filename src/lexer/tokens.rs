use std::fmt::Display;

#[derive(Debug, Clone, Copy)]

pub struct Location {
  pub column: usize,
  pub row: usize,
}

#[derive(Debug)]
pub enum Token {
  Keyword(String, Location),
  Identifier(String, Location),
  StringLiteral(String, Location),
  NumberLiteral(String, Location),
  Operator(String, Location),
  AssignOperator(String, Location),
  IncrementOperator(String, Location),
  Punctuation(String, Location),
  Delimiter(String, Location),
  WhiteSpace(String, Location),
  Invalid(char, Location),
  Reserved(String, Location),
}

impl Display for Location {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "({}:{})", self.row, self.column)
  }
}
