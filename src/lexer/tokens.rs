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
  Ignore,
  Invalid(char, Location),
}
