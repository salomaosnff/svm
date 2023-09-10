#[macro_export]
macro_rules! operator {
  ($pattern:pat $(if $guard:expr)? $(,)?) => {
    |op: &str| matches!(op, $pattern $(if $guard)?)
  };
}
