// Simple parser, which turns tokens into our internal encoding:

use encoding::Token;
use encoding::TokenValue;

use std::process;

use encoding::Block;
use encoding::Expression;
use encoding::List;
use encoding::Call;
use encoding::Definition;

pub fn parse_error(msg: String, pos: isize, line: &String) {
  println!("--- Parse error on line {} :\n--- {} :", pos, msg);
  println!("\"{}\"", line);
  process::exit(1);
}

fn get_token(tokens: &Vec<Token>, start: usize) -> &Token {
  if start > tokens.len() - 1 {
    parse_error("unexpected end of file; statement unterminated".to_string(),
                -1, &"EOF".to_string());
  }
  &tokens[start]
}

fn parse_params(tokens: &Vec<Token>, start: usize) ->
  (Option<Vec<String>>, usize) {
  let mut rc = Vec::new();
  let mut index = start;
  loop {
    let token = get_token(tokens, index);
    match token.value {
      TokenValue::CloseBracket => {
        index += 1;
        break;
      },
      TokenValue::ID(ref s) => {
        rc.push(s.clone());
        index += 1;
        let token = get_token(tokens, index);
        match token.value {
          TokenValue::CloseBracket => {
            // do nothing, next loop will catch it
          },
          _ => {
            return (None, 0);
          }
        }
      },
      _ => {
        return (None, 0);
      },
    }
  }
  (Some(rc), index)
}

fn parse_definition(tokens: &Vec<Token>, start: usize) ->
  (Option<Definition>, usize) {
  let token = get_token(tokens, start);
  match token.value {
    TokenValue::Colon => {
      // anonymous function with no parameters
      let (block, index) = parse_block(tokens, start + 1);
      (Some(Definition { id: "".to_string(), params: Vec::new(),
                         block: block }), index)
    },
    TokenValue::OpenBracket => {
      // anonymous function
      let (opt, index) = parse_params(tokens, start + 1);
      match opt {
        Some(params) => {
          let token = get_token(tokens, index);
          match token.value {
            TokenValue::Colon => {
              let (block, last) = parse_block(tokens, index + 1);
              (Some(Definition { id:"".to_string(), params: params,
                                 block: block }), last)
            },
            _ => (None, 0),
          }
        },
        None => (None, 0),
      }
    },
    TokenValue::ID(ref id) => {
      let mut index = start + 1;
      let token = get_token(tokens, index);
      match token.value {
        TokenValue::Colon => {
          index += 1;
          let (block, change) = parse_block(tokens, index);
          (Some(Definition { id: id.clone(), params: Vec::new(),
                             block: block }), change)
        },
        TokenValue::OpenBracket => {
          let (opt, change) = parse_params(tokens, index + 1);
          match opt {
            Some(params) => {
              index = change;
              let token = get_token(tokens, index);
              match token.value {
                TokenValue::Colon => {
                  index += 1;
                  let (block, last) = parse_block(tokens, index);
                  (Some(Definition { id: id.clone(), params: params,
                                     block: block }), last)
                },
                _ => (None, 0),
              }
            },
            None => (None, 0),
          }
        },
        _ => (None, 0),
      }
    },
    _ => (None, 0),
  }
}

fn parse_call(tokens: &Vec<Token>, start: usize) -> (Call, usize) {
  let mut token = get_token(tokens, start);
  let id = match token.value {
    TokenValue::ID(ref s) => s.clone(),
    _ => panic!("if you see this, there's a bug in the parser"),
  };
  let mut rc = Call { id: id, params: Vec::new() };
  let mut index = start + 1;
  token = get_token(tokens, index);
  match token.value {
    TokenValue::OpenBracket => {
      index += 1;
      loop {
      token = get_token(tokens, index);
        match token.value {
          TokenValue::CloseBracket => {
            index += 1;
            break;
          },
          _ => {
            let (param, change) = parse_next_expression(tokens, index);
            match param {
              Some(exp) => rc.params.push(exp),
              None =>  parse_error("expression or close paren expected".to_string(),
                                   token.lnum, &token.line),

            }
            index = change;
          }
        }
      }
    },
    _ => {
      // Do nothing, bare function call
    },
  }
  (rc, index)
}

fn parse_list(tokens: &Vec<Token>, start: usize) -> (List, usize) {
  let token = get_token(tokens, start + 1);
  match token.value {
    TokenValue::CloseBracket => (List {head: Box::new(Head::Empty)}, start + 2),
    _ => {
      let (item, index) = parse_next_expression(tokens, start + 1);
      match item {
        some(exp) => {
          let (tail, final) = parse_list(tokens, index);
          (List { head: Box:new(Head::Expression(item))
                  tail: Box::new(tail) }, final + 1)
        },
        None => parse_error("expression or close bracket expected".to_string(),
                            token.lnum, &token.line),
      }
    },
  }
}
fn parse_next_expression(tokens: &Vec<Token>, start: usize) ->
  (Option<Expression>, usize) {
  let token = get_token(tokens, start);
  match token.value {
    TokenValue::True => (Some(Expression::True), start + 1),
    TokenValue::False => (Some(Expression::False), start + 1),
    TokenValue::Integer(x) => (Some(Expression::Integer(x)), start + 1),
    TokenValue::Float(x) => (Some(Expression::Float(x)), start + 1),
    TokenValue::String(ref s) => (Some(Expression::String(s.clone())), start + 1),
    TokenValue::OpenBracket => {
      let (list, index) = parse_list(tokens, start);
      (Some(Expression::List(list)), index)
    },
    TokenValue::ID(_) => {
      let (opt, index) = parse_definition(tokens, start);
      match opt {
        Some(def) => {
          (Some(Expression::Definition(def)), index - 1)
        },
        None => {
          let (call, index) = parse_call(tokens, start);
          (Some(Expression::Call(call)), index)
        },
      }
    },
    TokenValue::Colon => {
      let (opt, index) = parse_definition(tokens, start);
      match opt {
        Some(def) => {
          (Some(Expression::Definition(def)), index - 1)
        },
        None => {
          parse_error("expected function definition, didn't get one".to_string(),
                      token.lnum, &token.line);
          (None, 0)
        },
      }
    },
    _ => (None, 0),
  }
}

fn parse_block(tokens: &Vec<Token>, start: usize) -> (Block, usize) {
  let mut rc = Block { expressions: Vec::new() };
  let mut index = start;
  loop {
    let (next, change) = parse_next_expression(tokens, index);
    match next {
      Some(value) => {
        index = change;
        // For debugging:
        println!("{:?}", value);
        rc.expressions.push(value);
      },
      None => {
        index += 1;
        break;
      },
    }
    let check = get_token(tokens, index);
    match check.value {
      TokenValue::Semicolon => {
        // do nothing
      },
      _ => {
        parse_error("semicolon expected after expression".to_string(),
                    check.lnum, &check.line);
      },
    }
    index += 1;
  }
  (rc, index)
}

pub fn parse(tokens: &Vec<Token>) -> Block {
  let (block, index) = parse_block(&tokens, 0);
  if index < tokens.len() {
    parse_error("syntax error, unexpected token".to_string(),
                tokens[index].lnum, &tokens[index].line);
  }
  block
}
