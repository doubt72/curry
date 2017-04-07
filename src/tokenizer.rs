// Super simple tokenizer/scanner:

use encoding::LineLookup;
use encoding::Token;
use encoding::TokenValue;

// TODO: this is for one call to parse_error; set up a more general utility?
use parser;

fn get_lnum(pos: usize, key: &LineLookup) -> usize {
  key.lnums[pos] + 1
}

fn get_line(pos: usize, key: &LineLookup) -> String {
  key.lines[key.lnums[pos]].clone()
}

fn next_token(chars: &Vec<char>, start: usize, key: &LineLookup) ->
  (Token, usize) {
  let reserved = [':', ';', '[', ']', '"', '#'];

  let mut index = start;
  let mut c = chars[index];
  while c.is_whitespace() {
    if index == chars.len() - 1 {
      // EOF is only returned with trailing whitespace (or closing comment), but
      // we need to return something when there's no "real" token left to return
      return (Token { value: TokenValue::EOF, lnum: -1, line: "".to_string() },
              index + 1);
    }
    index += 1;
    c = chars[index];
  }
  let from = index;
  let (value, pos) = match c {
    ':' => (TokenValue::Colon, index + 1),
    ';' => (TokenValue::Semicolon, index + 1),
    '[' => (TokenValue::OpenBracket, index + 1),
    ']' => (TokenValue::CloseBracket, index + 1),
    '#' => {
      index += 1;
      c = chars[index];
      // Comment; eat the rest of the line
      while index < chars.len() - 1 && c != '\n' && c != '\r' {
        index += 1;
        c = chars[index];
      }
      // This is a comment, so we return the next token after it
      return next_token(&chars, index, key)
    },
    '"' => {
      index += 1;
      c = chars[index];
      // Everything to next double-quote is string
      // TODO: string escapes
      while index < chars.len() - 1 && c != '"' {
        index += 1;
        c = chars[index];
      }
      let s = chars[from + 1..index].iter().cloned().collect();
      if c != '"' {
        parser::parse_error(format!("unterminated string in source: {}", s),
                            -1, &"EOF".to_string());
      }
      (TokenValue::String(s), index + 1)
    },
    _ => {
      while index < chars.len() - 1 && !c.is_whitespace() &&
        !reserved.contains(&c) {
        index += 1;
        c = chars[index];
      }
      let s:String = chars[from..index].iter().cloned().collect();
      if s == "true" {
        (TokenValue::True, index)
      } else if s == "false" {
        (TokenValue::False, index)
      } else {
        match s.parse::<i64>() {
          Ok(n) => (TokenValue::Integer(n), index),
          _ => {
            match s.parse::<f64>() {
              Ok(n) => (TokenValue::Float(n), index),
              _=> (TokenValue::ID(s), index),
            }
          },
        }
      }
    },
  };
  let token = Token { value: value, lnum: get_lnum(pos, key) as isize,
                      line: get_line(pos, key) };
  (token, pos)
}

pub fn build_line_key(chars: &Vec<char>) -> LineLookup {
  let mut key = LineLookup { lnums: Vec::new(), lines: Vec::new() };

  let mut line_num = 0;
  let mut start = 0;
  let mut current = 0;
  for i in 0..chars.len() {
    key.lnums.push(line_num);
    if chars[i] == '\n' || chars[i] == '\r' {
      let line = chars[start..current].iter().cloned().collect();
      key.lines.push(line);
      start = current + 1;
      line_num += 1;
    }
    current += 1;
  }
  key
}

pub fn tokenize(s: &str) -> Vec<Token> {
  let chars = s.chars().collect();
  let key = build_line_key(&chars);

  let mut tokens = Vec::new();

  let mut index = 0;
  while index < chars.len() {
    let (token, change) = next_token(&chars, index, &key);
    index = change;
    tokens.push(token);
  }
  tokens
}
