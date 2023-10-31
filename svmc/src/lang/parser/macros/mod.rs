#[macro_export]
macro_rules! operator {
  ($pattern:pat $(if $guard:expr)? $(,)?) => {
    |op: &str| matches!(op, $pattern $(if $guard)?)
  };
}

#[macro_export]
macro_rules! get_operator {
  ($lexer:expr, $pattern:pat $(if $guard:expr)? $(,)?) => {
    $lexer.consume_if(|t| matches!(t, Token::Punctuator(op, _) if matches!(op.as_str(), $pattern $(if $guard)?)))
  };
}

#[macro_export]
macro_rules! expect_operator {
  (lexer: Lexer, $pattern:pat $(if $guard:expr)? $(,)?) => {
    get_operator!(lexer, $pattern $(if $guard)?).expect(&format!("Expected operator {}", stringify!($pattern)))
  };
}
