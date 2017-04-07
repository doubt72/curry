use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;

use encoding::Token;
use encoding::TokenValue;

use encoding::Expression;
use encoding::List;

use encoding::Scope;
use encoding::Evaluation;
use encoding::Function;
use encoding::Exception;
use encoding::ExceptionType;

impl Debug for Token {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    let mut s = match self.value {
      TokenValue::Colon => "COLON".to_string(),
      TokenValue::Semicolon => "SEMICOLON".to_string(),
      TokenValue::True => "TRUE".to_string(),
      TokenValue::False => "FALSE".to_string(),
      TokenValue::OpenBracket => "OPENBRACKET".to_string(),
      TokenValue::CloseBracket => "CLOSEBRACKET".to_string(),
      TokenValue::ID(ref x) => "ID:".to_string() + &x,
      TokenValue::Integer(ref x) => "INTEGER:".to_string() + &x.to_string(),
      TokenValue::Float(ref x) => "FLOAT:".to_string() + &x.to_string(),
      TokenValue::String(ref x) => "STRING:".to_string() + &x,
      TokenValue::EOF => "EOF".to_string(),
    };
    s += &format!("/[{}:{}]", self.lnum, self.line);
    write!(f, "{}", s)
  }
}

impl Debug for Expression {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    let s = match self {
      &Expression::True => "TRUE".to_string(),
      &Expression::False => "FALSE".to_string(),
      &Expression::Integer(ref x) => "INTEGER:".to_string() + &x.to_string(),
      &Expression::Float(ref x) => "FLOAT:".to_string() + &x.to_string(),
      &Expression::String(ref x) => "STRING:".to_string() + &x,
      &Expression::List(ref x) => {
        format!("{:?}", x)
      },
      &Expression::Call(ref x) => {
        format!("CALL:{}:{:?}", x.id, x.param)
      },
      &Expression::Definition(ref x) => {
        let mut s2 = "DEFINITION:".to_string() + &x.id;
        for i in &x.block.expressions {
          s2 += &format!("{:?};", i);
        }
        s2
      },
    };
    write!(f, "{}", s)
  }
}

impl Debug for List {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    let mut s = "LIST:[ ".to_string();
    for i in &self.items {
      s += &format!("{:?} ", i);
    }
    s += "]";
    write!(f, "{}", s)
  }
}

impl Debug for Scope {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    let mut s = "SCOPE:".to_string();
    for (id, f) in &self.bindings {
      s += &format!(" {}:{:?}", id, f);
    }
    write!(f, "{}", s)
  }
}

impl Debug for Evaluation {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    let s = match self {
      &Evaluation::True => "TRUE".to_string(),
      &Evaluation::False => "FALSE".to_string(),
      &Evaluation::Integer(ref x) => "INTEGER:".to_string() + &x.to_string(),
      &Evaluation::Float(ref x) => "FLOAT:".to_string() + &x.to_string(),
      &Evaluation::String(ref x) => "STRING:".to_string() + &x,
      &Evaluation::List(ref x) => {
        let mut s2 = "LIST:[ ".to_string();
        for i in &x.items {
          s2 += &format!("{:?} ", i);
        }
        s2 += "]";
        s2
      },
      &Evaluation::Exception(ref x) => {
        let mut s2 = format!("EXCEPTION:[{}, ", x.flavor);
        s2 += &format!("{}, ", x.payload);
        let mut stack = Vec::new();
        for i in &x.stack {
          stack.push(i.clone());
        }
        s2 += &stack.join(", ");
        s2 += "]]";
        s2
      },
      &Evaluation::Function(ref x) => {
        format!("FUNCTION:{:?}", x)
      },
    };
    write!(f, "{}", s)
  }
}

impl Debug for Function {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    let mut s = ":<".to_string();
    for e in &self.block.expressions {
      s += &format!("{:?};", e);
    }
    s += ">";
    write!(f, "{}", s)
  }
}

impl Display for Evaluation {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    let s = match self {
      &Evaluation::True => "true".to_string(),
      &Evaluation::False => "false".to_string(),
      &Evaluation::Integer(x) => x.to_string(),
      &Evaluation::Float(x) => x.to_string(),
      &Evaluation::String(ref x) => format!("\"{}\"", x),
      &Evaluation::List(ref x) => {
        let mut s2 = "[".to_string();
        let mut items = Vec::new();
        for i in &x.items {
          items.push(format!("{}", i));
        }
        s2 += &items.join(" ");
        s2 += "]";
        s2
      },
      &Evaluation::Exception(ref x) => {
        let mut s2 = format!("[{}, ", x.flavor);
        s2 += &format!("{}, ", x.payload);
        let mut stack = Vec::new();
        for i in &x.stack {
          stack.push(i.clone());
        }
        s2 += &stack.join(", ");
        s2 += "]]";
        s2
      },
      &Evaluation::Function(_) => {
        ":<...>".to_string()
      },
    };
    write!(f, "{}", s)
  }
}

impl Display for Exception {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    let mut s = format!("\nRUNTIME EXCEPTION: {}\n{}:\n\n  calling context:\n",
                        self.flavor.to_string().to_uppercase(), self.payload);
    let mut n = self.stack.len();
    for i in &self.stack {
      s += &format!("   -- called from function {}: {}\n", n - 1, i);
      n -= 1;
    }
    write!(f, "{}", s)
  }
}

impl Display for ExceptionType {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    let s = match self {
      &ExceptionType::Return => "return".to_string(),
      &ExceptionType::Error => "error".to_string(),
      &ExceptionType::ArgError => "parameter length".to_string(),
      &ExceptionType::ParseError => "parse error".to_string(),
      &ExceptionType::TypeError => "type error".to_string(),
      &ExceptionType::TypeMismatch => "type mismatch".to_string(),
      &ExceptionType::DivByZero => "division by zero".to_string(),
      &ExceptionType::RuntimeError => "runtime error".to_string(),
      &ExceptionType::UndefError => "undefined function".to_string(),
      &ExceptionType::RedefError => "redefinition error".to_string(),
    };
    write!(f, "{}", s)
  }
}
