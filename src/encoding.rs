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

pub struct List {
  pub head: Option<Box<Expression>>,
  pub tail: Option<Box<List>>
}

pub struct Call {
  pub id: String,
  pub params: Vec<Expression>
}

pub struct Definition {
  pub id: String,
  pub params: Vec<String>,
  pub block: Block
}

pub struct Scope {
  pub bindings: HashMap<String, FunctionOrValue>
}

pub enum FunctionOrValue {
  Function(Function), Value(Evaluation)
}

pub enum Evaluation {
  True, False, Integer(i64), Float(f64), String(String), List(ListEval),
  Function(Function), Exception(Exception)
}

pub struct ListEval {
  pub head: Option<Box<Evaluation>>,
  pub tail: Option<Box<ListEval>>
}

pub struct Function {
  pub params: Vec<String>,
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
