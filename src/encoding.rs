// Our internal representation of the language

use std::collections::HashMap;

pub struct LineLookup {
  pub lnums: Vec<usize>,
  pub lines: Vec<String>
}

pub struct Token {
  pub value: TokenValue,
  pub lnum: isize,
  pub line: String
}

pub enum TokenValue {
  Colon, Semicolon, OpenBracket, CloseBracket,
  ID(String), Integer(i64), Float(f64), String(String),
  True, False, EOF
}

pub struct Block {
  pub expressions: Vec<Expression>
}

pub enum Expression {
  True, False, Integer(i64), Float(f64), String(String), List(List),
  Call(Call), Definition(Definition)
}

// On a high level, we treat lists like S-expressions, but due to some major
// awkwardness in Rust, it's actually much, much easier to implement lists as a
// vector.  Not because the logic is easier (it's not...  Well, ceremony aside,
// anyway), but because the option-box pattern (besides being inherently
// awkward) is well nigh unusable for certain use cases
pub struct List {
  pub items: Vec<Expression>
}

pub struct Call {
  pub id: String,
  pub param: List
}

pub struct Definition {
  pub id: String,
  pub block: Block
}

pub struct Scope {
  pub bindings: HashMap<String, Function>,
  pub param: ListEval
}

pub enum Evaluation {
  True, False, Integer(i64), Float(f64), String(String), List(ListEval),
  Function(Function), Exception(Exception)
}

pub struct ListEval {
  pub items: Vec<Evaluation>
}

pub struct Function {
  pub block: Block
}

pub struct Exception {
  pub flavor: ExceptionType,
  pub payload: Box<Evaluation>,
  pub stack: Vec<String>
}

pub enum ExceptionType {
  Return, Error, ArgError, ParseError, TypeError, TypeMismatch, DivByZero,
  RuntimeError, UndefError, RedefError
}
