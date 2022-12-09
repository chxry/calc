use std::io::{self, Write};
use std::slice::Iter;
use std::iter::Peekable;
use std::collections::HashMap;
use std::f64::consts;

fn main() -> Result<(), io::Error> {
  println!("calc! - type 'help'");
  let mut vars = HashMap::from([("pi".to_string(), consts::PI), ("e".to_string(), consts::E)]);
  loop {
    print!("> ");
    io::stdout().flush()?;
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    match line.as_str().trim() {
      "ls" => {
        for (k, v) in vars.iter() {
          println!("{}: {}", k, v);
        }
      }
      "help" => {
        println!("help:");
        println!("'ls' to show variables.");
        println!("'+,-,*,/,^' available operators.");
      }
      _ => {
        let ans = calc(line, &vars);
        match ans {
          Ok(n) => {
            vars.insert("ans".to_string(), n);
            println!("{}", n)
          }
          Err(e) => println!("{}", e.fmt()),
        }
      }
    }
  }
}

fn calc(str: String, vars: &HashMap<String, f64>) -> Result<f64, Error> {
  let mut chars = str.chars().peekable();
  let mut tokens = Vec::new();
  while let Some(c) = chars.next() {
    if c.is_whitespace() {
      continue;
    }
    match c {
      '0'..='9' => {
        let mut num = c.to_digit(10).unwrap() as _;
        let mut dec = 1.0;
        while let Some(c) = chars.peek() {
          match c.to_digit(10) {
            Some(d) => {
              if dec == 1.0 {
                num = num * 10.0 + d as f64;
              } else {
                num += d as f64 * dec;
                dec /= 10.0;
              }
              chars.next();
            }
            None => {
              if *c == '.' {
                if dec < 1.0 {
                  return Err(Error::UnexpectedToken(*c));
                }
                dec /= 10.0;
                chars.next();
              } else {
                break;
              }
            }
          }
        }
        tokens.push(Token::Num(num));
      }
      c if c.is_alphabetic() => {
        let mut s = c.to_string();
        while let Some(c) = chars.peek() {
          if c.is_alphabetic() {
            s.push(*c);
            chars.next();
          } else {
            break;
          }
        }
        tokens.push(Token::Var(s));
      }
      '+' => tokens.push(Token::Add),
      '-' => tokens.push(Token::Sub),
      '*' => tokens.push(Token::Mul),
      '/' => tokens.push(Token::Div),
      '^' => tokens.push(Token::Pow),
      _ => return Err(Error::UnexpectedToken(c)),
    }
  }
  let mut tokens = tokens.iter().peekable();
  let mut num = get_num(&mut tokens, vars).unwrap_or(0.0);
  while let Some(t) = tokens.next() {
    let n = get_num(&mut tokens, vars)?;
    match t {
      Token::Add => num += n,
      Token::Sub => num -= n,
      Token::Mul => num *= n,
      Token::Div => num /= n,
      Token::Pow => num = num.powf(n),
      _ => return Err(Error::ExpectedOperator((*t).clone())),
    }
  }
  Ok(num)
}

fn get_num(tokens: &mut Peekable<Iter<Token>>, vars: &HashMap<String, f64>) -> Result<f64, Error> {
  match tokens.peek() {
    Some(Token::Num(n)) => {
      tokens.next();
      Ok(*n)
    }
    Some(Token::Var(v)) => {
      tokens.next();
      Ok(*vars.get(v).unwrap_or(&0.0))
    }
    t => Err(Error::ExpectedExpr(t.cloned().cloned())),
  }
}

#[derive(Clone)]
enum Token {
  Num(f64),
  Var(String),
  Add,
  Sub,
  Mul,
  Div,
  Pow,
}

fn fmt_token(token: Option<&Token>) -> String {
  match token {
    Some(t) => match t {
      Token::Num(n) => n.to_string(),
      Token::Var(s) => s.clone(),
      Token::Add => "+".to_string(),
      Token::Sub => "-".to_string(),
      Token::Mul => "*".to_string(),
      Token::Div => "/".to_string(),
      Token::Pow => "^".to_string(),
    },
    None => "none".to_string(),
  }
}

enum Error {
  UnexpectedToken(char),
  ExpectedOperator(Token),
  ExpectedExpr(Option<Token>),
}

impl Error {
  fn fmt(&self) -> String {
    match self {
      Error::UnexpectedToken(c) => format!("unexpected token '{}'", c),
      Error::ExpectedOperator(t) => format!("expected operator, found '{}'.", fmt_token(Some(t))),
      Error::ExpectedExpr(t) => format!("expected expression, found '{}'.", fmt_token(t.as_ref())),
    }
  }
}
