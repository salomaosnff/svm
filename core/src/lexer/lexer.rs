use super::{code::Code, consumer::Consumer, Location, Token};

pub type Lexer = Consumer<Token>;

struct Tokenizer {
  code: Consumer<char>,
  location: Location,
}

impl Tokenizer {
  pub fn new(consumer: Consumer<char>) -> Tokenizer {
    Tokenizer {
      code: consumer,
      location: Location { column: 0, row: 1 },
    }
  }

  fn next_char(&mut self) -> Option<char> {
    return match self.code.consume() {
      Some('\n') => {
        self.location.column = 0;
        self.location.row += 1;

        Some('\n')
      }
      Some(c) => {
        self.location.column += 1;

        Some(c)
      }
      None => None,
    };
  }

  fn read_next_token(&mut self) -> Option<Token> {
    let c = self.next_char()?;

    return match c {
      '\n' => Some(Token::End(self.location)),
      // White spaces
      c if c.is_whitespace() => Some(Token::WhiteSpace(c.to_string(), self.location)),

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
          "let" | "this" | "null" | "true" | "false" | "function" | "const" | "return" | "if"
          | "else" => Some(Token::Keyword(id, self.location)),
          "await" | "break" | "case" | "catch" | "class" | "continue" | "debugger" | "default"
          | "delete" | "do" | "enum" | "export" | "extends" | "finally" | "for" | "import"
          | "in" | "instanceof" | "new" | "super" | "switch" | "throw" | "try" | "typeof"
          | "var" | "void" | "while" | "with" | "yield  | " => {
            Some(Token::Reserved(id, self.location))
          }
          _ => Some(Token::IdentifierName(id, self.location)),
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

      // Puntuactors
      '{' | '}' | '(' | ')' | '[' | ']' | ':' | '~' | ',' | ';' => {
        Some(Token::Punctuator(c.to_string(), self.location))
      }

      '.' => {
        let mut p = String::from(c);

        if self.code.consume_if(|c| c.to_string() == ".").is_some() {
          p.push('.');
        }

        Some(Token::Punctuator(p, self.location))
      }

      '%' | '^' | '/' => {
        let mut p = String::from(c);

        if self.code.consume_if(|c| c.to_string() == "=").is_some() {
          p.push('=');
        }

        Some(Token::Punctuator(p, self.location))
      }

      '<' => {
        let mut p = String::from(c);

        if self.code.consume_if(|c| c.to_string() == "<").is_some() {
          p.push('<');

          if self.code.consume_if(|c| c.to_string() == "<").is_some() {
            p.push('<');
          }
        }

        if self.code.consume_if(|c| c.to_string() == "=").is_some() {
          p.push('=');
        }

        Some(Token::Punctuator(p, self.location))
      }

      '>' => {
        let mut p = String::from(c);

        if self.code.consume_if(|c| c.to_string() == ">").is_some() {
          p.push('>');

          if self.code.consume_if(|c| c.to_string() == ">").is_some() {
            p.push('>');

            if self.code.consume_if(|c| c.to_string() == ">").is_some() {
              p.push('>');
            }
          }
        }

        if self.code.consume_if(|c| c.to_string() == "=").is_some() {
          p.push('=');
        }

        Some(Token::Punctuator(p, self.location))
      }

      '+' | '-' => {
        let mut p = String::from(c);

        if self.code.consume_if(|ch| *ch == c).is_some() {
          p.push(c);
        } else if self.code.consume_if(|ch| *ch == '=').is_some() {
          p.push('=');
        }

        Some(Token::Punctuator(p, self.location))
      }

      '*' => {
        let mut p = String::from(c);

        if self.code.consume_if(|c| c.to_string() == "*").is_some() {
          p.push('*');
        }

        if self.code.consume_if(|c| c.to_string() == "=").is_some() {
          p.push('=');
        }

        Some(Token::Punctuator(p, self.location))
      }

      '&' | '|' => {
        let mut p = String::from(c);

        if self.code.consume_if(|ch| *ch == c).is_some() {
          p.push(c);
        }

        if self.code.consume_if(|ch| ch.to_string() == "=").is_some() {
          p.push('=');
        }

        Some(Token::Punctuator(p, self.location))
      }

      '!' => {
        let mut p = String::from(c);

        if self.code.consume_if(|c| c.to_string() == "=").is_some() {
          p.push('=');

          if self.code.consume_if(|c| c.to_string() == "=").is_some() {
            p.push('=');
          }
        }

        Some(Token::Punctuator(p, self.location))
      }

      '?' => {
        let mut p = String::from(c);

        if self.code.consume_if(|c| c.to_string() == ".").is_some() {
          p.push('.');
        } else if self.code.consume_if(|c| c.to_string() == "?").is_some() {
          p.push('?');

          if self.code.consume_if(|c| c.to_string() == "=").is_some() {
            p.push('=');
          }
        }

        Some(Token::Punctuator(p, self.location))
      }

      '=' => {
        let mut p = String::from(c);

        if self.code.consume_if(|c| c.to_string() == "=").is_some() {
          p.push('=');

          if self.code.consume_if(|c| c.to_string() == "=").is_some() {
            p.push('=');
          }
        } else if self.code.consume_if(|c| c.to_string() == ">").is_some() {
          p.push('>');
        }

        Some(Token::Punctuator(p, self.location))
      }

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
                .consume_while(|ch| matches!(ch, '0' | '1' | '_'))
                .into_iter()
                .collect::<String>(),

              // Octal Literal
              'o' => self
                .code
                .consume_while(|ch| {
                  matches!(ch, '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '_')
                })
                .into_iter()
                .collect::<String>(),

              // Hex literal
              'x' => self
                .code
                .consume_while(|ch| ch.is_ascii_hexdigit() || *ch == '_')
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
              .consume_while(|c| c.is_numeric() || *c == '.')
              .into_iter()
              .collect::<String>(),
          );
        }

        return Some(Token::NumericLiteral(number_literal, self.location));
      }

      // Invalid
      c => Some(Token::Invalid(c, self.location)),
    };
  }
}

impl Iterator for Tokenizer {
  type Item = Token;

  fn next(&mut self) -> Option<Self::Item> {
    loop {
      match self.read_next_token() {
        Some(Token::Invalid(ch, loc)) => panic!("Token \"{}\" inválido em {}", ch, loc),
        Some(Token::Reserved(ch, loc)) => {
          panic!("Token \"{}\" em {} é reservado!", ch, loc)
        }
        Some(Token::WhiteSpace(_, _)) => continue,
        Some(token) => return Some(token),
        _ => return None,
      }
    }
  }
}

pub fn from_code(code: Code) -> Lexer {
  return Consumer::new(Tokenizer::new(code));
}
