use super::{consumer::Consumer, Location, Token};

pub struct Lexer {
  code: Consumer<char>,
  location: Location,
}

impl Lexer {
  pub fn new(consumer: Consumer<char>) -> Lexer {
    Lexer {
      code: consumer,
      location: Location { column: 0, row: 1 },
    }
  }

  fn read_next_token(&mut self) -> Option<Token> {
    let c = self.code.consume()?;

    return match c {
      // White spaces
      c if c.is_whitespace() => {
        if c == '\n' {
          self.location.column = 0;
          self.location.row += 1;
        }

        self.location.column += 1;

        return Some(Token::WhiteSpace(c.to_string(), self.location));
      }

      // Identifiers
      c if c.is_alphabetic() || c == '$' || c == '_' => {
        let mut id = String::from(c);

        id.push_str(
          &self
            .code
            .consume_while(|c| c.is_alphanumeric() || c == &'$' || c == &'_')
            .into_iter()
            .collect::<String>(),
        );

        if id.len() <= 0 {
          return None;
        }

        return match id.as_str() {
          "let" => Some(Token::Keyword(id, self.location)),
          "await" | "break" | "case" | "catch" | "class" | "const" | "continue" | "debugger"
          | "default" | "delete" | "do" | "else" | "enum" | "export" | "extends" | "false"
          | "finally" | "for" | "function" | "if" | "import" | "in" | "instanceof" | "new"
          | "null" | "return" | "super" | "switch" | "this" | "throw" | "true" | "try"
          | "typeof" | "var" | "void" | "while" | "with" | "yield  | " => {
            Some(Token::Reserved(id, self.location))
          }
          _ => Some(Token::Identifier(id, self.location)),
        };
      }

      // String Literal
      '"' | '\'' => {
        let delimiter = c;
        let mut str_literal = String::from(delimiter);

        loop {
          let str_char = self.code.consume().expect("Fim inesperado da String.");

          if str_char == delimiter {
            str_literal.push(delimiter);
            break;
          }

          if str_char == '\\' {
            let escaped_char = self.code.consume().expect("Fim inesperado da String");

            str_literal.push(escaped_char);

            continue;
          }

          str_literal.push(str_char)
        }

        Some(Token::StringLiteral(str_literal, self.location))
      }

      // Operators
      '+' | '-' | '*' | '/' | '%' | '|' | '&' | '^' | '~' | '<' | '>' | '!' | '?' => {
        let mut op = String::from(c);

        // ++, --
        if (op == "+" || op == "-") && self.code.consume_if(|c| c.to_string() == op).is_some() {
          return Some(Token::IncrementOperator(op.repeat(2), self.location));
        }

        // **, ||, &&, <<, >>, ??
        if (op == "*" || op == "|" || op == "&" || op == "<" || op == ">" || op == "?")
          && self.code.consume_if(|c| c.to_string() == op).is_some()
        {
          op.push_str(&op.clone());
        }

        let equals_char = self.code.consume_if(|c| c.to_string() == "=");

        // >=, <=, !=
        if (op == "<" || op == ">" || op == "!") && equals_char.is_some() {
          op.push(equals_char.unwrap());
          return Some(Token::Operator(op, self.location));
        }

        // +=, -=, *=, /=, %=, |=, &=, ^=, ~=, **=, <<=, >>=, &&=, ||=, ??=
        if (op == "+"
          || op == "-"
          || op == "*"
          || op == "/"
          || op == "%"
          || op == "|"
          || op == "&"
          || op == "^"
          || op == "~"
          || op == "**"
          || op == "<<"
          || op == ">>"
          || op == "&&"
          || op == "||"
          || op == "??")
          && equals_char.is_some()
        {
          op.push(equals_char.unwrap());
          return Some(Token::AssignOperator(op, self.location));
        }

        // ?, !
        if op == "?" || op == "!" {
          return Some(Token::Punctuation(op, self.location));
        }

        return Some(Token::Operator(op, self.location));
      }

      // :
      ':' => Some(Token::Punctuation(c.to_string(), self.location)),

      // Number literal
      c if c.is_numeric() => {
        let mut number_literal = String::from(c);

        if c == '0' {
          let base = self.code.consume_if(|c| matches!(c, 'b' | 'o' | 'x'));

          if base.is_some() {
            number_literal.push(base.unwrap());

            let value = match base? {
              // Binary Literal
              'b' => self
                .code
                .consume_while(|ch| matches!(ch, '0' | '1'))
                .into_iter()
                .collect::<String>(),

              // Octal Literal
              'o' => self
                .code
                .consume_while(|ch| matches!(ch, '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7'))
                .into_iter()
                .collect::<String>(),

              // Hex literal
              'x' => self
                .code
                .consume_while(|ch| ch.is_ascii_hexdigit())
                .into_iter()
                .collect::<String>(),

              _ => panic!("Número inválido!"),
            };

            if value.len() <= 0 {
              panic!("Número inválido!")
            }

            number_literal.push_str(value.as_str());
          } else {
            number_literal.push_str(
              &self
                .code
                .consume_while(|c| c.is_numeric())
                .into_iter()
                .collect::<String>(),
            );
          }
        } else {
          number_literal.push_str(
            &self
              .code
              .consume_while(|c| c.is_numeric())
              .into_iter()
              .collect::<String>(),
          );
        }

        return Some(Token::NumberLiteral(number_literal, self.location));
      }

      // Punctuation
      '.' | ',' | ';' => Some(Token::Punctuation(c.to_string(), self.location)),

      // Delimiter
      '(' | ')' | '[' | ']' | '{' | '}' => Some(Token::Delimiter(c.to_string(), self.location)),

      // Invalid
      c => Some(Token::Invalid(c, self.location)),
    };
  }
}

impl Iterator for Lexer {
  type Item = Token;

  fn next(&mut self) -> Option<Self::Item> {
    loop {
      match self.read_next_token() {
        Some(Token::Invalid(ch, loc)) => panic!("Token \"{}\" inválido em {}", ch, loc),
        Some(Token::Reserved(ch, loc)) => panic!("Token \"{}\" em {} é reservado!", ch, loc),
        Some(Token::WhiteSpace(_, _)) => continue,
        Some(token) => return Some(token),
        _ => return None,
      }
    }
  }
}

pub fn from_code(code: Consumer<char>) -> Consumer<Token> {
  return Consumer::new(Lexer::new(code));
}
