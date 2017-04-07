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

fn parse_definition(tokens: &Vec<Token>, start: usize) ->
  (Option<Definition>, usize) {
  let token = get_token(tokens, start);
  match token.value {
    TokenValue::Colon => {
      let (block, index) = parse_block(tokens, start + 1);
      (Some(Definition { id: "".to_string(), block: block }), index)
    },
    TokenValue::ID(ref id) => {
      let mut index = start + 1;
      let token = get_token(tokens, index);
      match token.value {
        TokenValue::Colon => {
          index += 1;
          let (block, change) = parse_block(tokens, index);
          (Some(Definition { id: id.clone(), block: block }), change)
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
  let mut rc = Call { id: id, param: List { items: Vec::new() }};
  let mut index = start + 1;
  token = get_token(tokens, index);
  match token.value {
    TokenValue::OpenBracket => {
      let (list, change) = parse_list(tokens, index);
      index = change;
      rc.param = list;
    },
    _ => {
      // Do nothing, bare function call
    },
  }
  (rc, index)
}

fn parse_list(tokens: &Vec<Token>, start: usize) -> (List, usize) {
  let mut index = start + 1;
  let mut rc = List { items: Vec::new() };
  loop {
    let token = get_token(tokens, index);
    match token.value {
      TokenValue::CloseBracket => {
        break;
      },
      _ => {
        let (item, change) = parse_next_expression(tokens, index);
        index = change;
        match item {
          Some(exp) => {
            rc.items.push(exp);
          },
          None => {
            parse_error("expression or close bracket expected".to_string(),
                        token.lnum, &token.line);
          },
        }
      },
    }
  }
  (rc, index + 1)
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
          parse_error("expected function definition after colon, didn't get one".to_string(),
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
