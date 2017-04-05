// Primitive functions

use evaluator;

use encoding::Evaluation;
use encoding::ListEval;
use encoding::Exception;
use encoding::ExceptionType;

fn expect_args(count: usize, params: &Vec<Evaluation>, id: &String) ->
  Option<Evaluation> {
  if count != params.len() {
    Some(evaluator::exception(ExceptionType::ArgError, id,
                              format!("expected argument list of length {} but got {}",
                                      count,
                                      params.len())))
  } else {
    None
  }
}

// TODO: break this up into functions?  Could abstract this substantially, too
pub fn system_functions(id: String, params: Vec<Evaluation>) -> Evaluation {
  if id != "?" && id != "catch" {
    for p in &params {
      match p {
        &Evaluation::Exception(_) => { return p.clone(); },
        _ => {
          // Not an exception, move along
        },
      }
    }
  }
  match &*id {
    // Type Conversion
    "int" => {
      match expect_args(1, &params, &id) {
        Some(e) => e,
        None => {
          match params[0] {
            Evaluation::Float(x) => Evaluation::Integer(x as i64),
            Evaluation::String(ref s) => {
              match s.parse::<i64>() {
                Ok(n) => Evaluation::Integer(n),
                _ => evaluator::exception(ExceptionType::ParseError, &id,
                                          format!("unable to parse string: {}", s)),
              }
            },
            _ => evaluator::exception(ExceptionType::TypeError, &id,
                                      "float or string argument expected".to_string()),
          }
        },
      }
    },
    "float" => {
      match expect_args(1, &params, &id) {
        Some(e) => e,
        None => {
          match params[0] {
            Evaluation::Integer(x) => Evaluation::Float(x as f64),
            Evaluation::String(ref s) => {
              match s.parse::<f64>() {
                Ok(n) => Evaluation::Float(n),
                _ => evaluator::exception(ExceptionType::ParseError, &id,
                                          format!("unable to parse string: {}", s)),
              }
            },
            _ => evaluator::exception(ExceptionType::TypeError, &id,
                                      "int or string argument expected".to_string()),
          }
        },
      }
    },
    "string" => {
      match expect_args(1, &params, &id) {
        Some(e) => e,
        None => Evaluation::String(format!("{}", params[0])),
      }
    },
    // IO
    ">>" => {
      match expect_args(1, &params, &id) {
        Some(e) => e,
        None => {
          match params[0] {
            Evaluation::String(ref s) => {
              println!("{}", s);
              // TODO: return empty list instead
              Evaluation::False
            },
            _ => evaluator::exception(ExceptionType::TypeError, &id,
                                      "string argument expected".to_string()),
          }
        },
      }
    },
    // MATH (plus appending things)
    "+" => {
      match expect_args(2, &params, &id) {
        Some(e) => e,
        None => {
          match params[0] {
            Evaluation::Integer(x) => {
              match params[1] {
                Evaluation::Integer(y) => Evaluation::Integer(x + y),
                Evaluation::Float(y) => Evaluation::Float(x as f64 + y),
                _ => evaluator::exception(ExceptionType::TypeMismatch, &id,
                                          "mismatched argument types".to_string()),
              }
            },
            Evaluation::Float(x) => {
              match params[1] {
                Evaluation::Integer(y) => Evaluation::Float(x + y as f64),
                Evaluation::Float(y) => Evaluation::Float(x + y),
                _ => evaluator::exception(ExceptionType::TypeMismatch, &id,
                                          "mismatched argument types".to_string()),
              }
            },
            Evaluation::String(ref s) => {
              match params[1] {
                Evaluation::String(ref t) => {
                  let mut rc = s.clone();
                  rc += &t.clone();
                  Evaluation::String(rc)
                },
                _ => evaluator::exception(ExceptionType::TypeMismatch, &id,
                                          "mismatched argument types".to_string()),
              }
            },
            Evaluation::List(ref list) => {
              match params[1] {
                Evaluation::List(ref list2) => {
                  let mut rc = ListEval { items: Vec::new() };
                  for i in &list.items {
                    rc.items.push(i.clone());
                  }
                  for i in &list2.items {
                    rc.items.push(i.clone());
                  }
                  Evaluation::List(rc)
                },
                _ => {
                  let mut rc = ListEval { items: Vec::new() };
                  for i in &list.items {
                    rc.items.push(i.clone());
                  }
                  rc.items.push(params[1].clone());
                  Evaluation::List(rc)
                },
              }
            },
            _ => evaluator::exception(ExceptionType::TypeError, &id,
                                      "numbers, strings, or list arguments expected".to_string()),
          }
        },
      }
    },
    "-" => {
      match expect_args(2, &params, &id) {
        Some(e) => e,
        None => {
          match params[0] {
            Evaluation::Integer(x) => {
              match params[1] {
                Evaluation::Integer(y) => Evaluation::Integer(x - y),
                Evaluation::Float(y) => Evaluation::Float(x as f64 - y),
                _ => evaluator::exception(ExceptionType::TypeError, &id,
                                          "numeric arguments expected".to_string()),
              }
            },
            Evaluation::Float(x) => {
              match params[1] {
                Evaluation::Integer(y) => Evaluation::Float(x - y as f64),
                Evaluation::Float(y) => Evaluation::Float(x - y),
                _ => evaluator::exception(ExceptionType::TypeError, &id,
                                          "numeric arguments expected".to_string()),
              }
            },
            _ => evaluator::exception(ExceptionType::TypeError, &id,
                                      "numeric arguments expected".to_string()),
          }
        },
      }
    },
    "*" => {
      match expect_args(2, &params, &id) {
        Some(e) => e,
        None => {
          match params[0] {
            Evaluation::Integer(x) => {
              match params[1] {
                Evaluation::Integer(y) => Evaluation::Integer(x * y),
                Evaluation::Float(y) => Evaluation::Float(x as f64 * y),
                _ => evaluator::exception(ExceptionType::TypeError, &id,
                                          "numeric arguments expected".to_string()),
              }
            },
            Evaluation::Float(x) => {
              match params[1] {
                Evaluation::Integer(y) => Evaluation::Float(x * y as f64),
                Evaluation::Float(y) => Evaluation::Float(x * y),
                _ => evaluator::exception(ExceptionType::TypeError, &id,
                                          "numeric arguments expected".to_string()),
              }
            },
            _ => evaluator::exception(ExceptionType::TypeError, &id,
                                      "numeric arguments expected".to_string()),
          }
        },
      }
    },
    "/" => {
      match expect_args(2, &params, &id) {
        Some(e) => e,
        None => {
          match params[0] {
            // TODO: handle division by zero with proper error
            Evaluation::Integer(x) => {
              match params[1] {
                Evaluation::Integer(y) => Evaluation::Integer(x / y),
                Evaluation::Float(y) => Evaluation::Float(x as f64 / y),
                _ => evaluator::exception(ExceptionType::TypeError, &id,
                                          "numeric arguments expected".to_string()),
              }
            },
            Evaluation::Float(x) => {
              match params[1] {
                Evaluation::Integer(y) => Evaluation::Float(x / y as f64),
                Evaluation::Float(y) => Evaluation::Float(x / y),
                _ => evaluator::exception(ExceptionType::TypeError, &id,
                                          "numeric arguments expected".to_string()),
              }
            },
            _ => evaluator::exception(ExceptionType::TypeError, &id,
                                      "numeric arguments expected".to_string()),
          }
        },
      }
    },
    "%" => {
      match expect_args(2, &params, &id) {
        Some(e) => e,
        None => {
          match params[0] {
            // TODO: handle division by zero with proper error
            Evaluation::Integer(x) => {
              match params[1] {
                Evaluation::Integer(y) => Evaluation::Integer(x % y),
                _ => evaluator::exception(ExceptionType::TypeError, &id,
                                          "integer arguments expected".to_string()),
              }
            },
            _ => evaluator::exception(ExceptionType::TypeError, &id,
                                      "integer arguments expected".to_string()),
          }
        },
      }
    },
    // BOOLEAN
    "!" => {
      match expect_args(1, &params, &id) {
        Some(e) => e,
        None => {
          match params[0] {
            Evaluation::True => Evaluation::False,
            Evaluation::False => Evaluation::True,
            _ => evaluator::exception(ExceptionType::TypeError, &id,
                                      "boolean argument expected".to_string()),
          }
        },
      }
    }
    "&" => {
      match expect_args(2, &params, &id) {
        Some(e) => e,
        None => {
          match params[0] {
            Evaluation::True => {
              match params[1] {
                Evaluation::True => Evaluation::True,
                Evaluation::False => Evaluation::False,
                _ => evaluator::exception(ExceptionType::TypeError, &id,
                                          "boolean arguments expected".to_string()),
              }
            },
            Evaluation::False => {
              match params[1] {
                Evaluation::True => Evaluation::False,
                Evaluation::False => Evaluation::False,
                _ => evaluator::exception(ExceptionType::TypeError, &id,
                                          "boolean arguments expected".to_string()),
              }
            },
            _ => evaluator::exception(ExceptionType::TypeError, &id,
                                      "boolean arguments expected".to_string()),
          }
        },
      }
    },
    "|" => {
      match expect_args(2, &params, &id) {
        Some(e) => e,
        None => {
          match params[0] {
            Evaluation::True => {
              match params[1] {
                Evaluation::True => Evaluation::True,
                Evaluation::False => Evaluation::True,
                _ => evaluator::exception(ExceptionType::TypeError, &id,
                                          "boolean arguments expected".to_string()),
              }
            },
            Evaluation::False => {
              match params[1] {
                Evaluation::True => Evaluation::True,
                Evaluation::False => Evaluation::False,
                _ => evaluator::exception(ExceptionType::TypeError, &id,
                                          "boolean arguments expected".to_string()),
              }
            },
            _ => evaluator::exception(ExceptionType::TypeError, &id,
                                      "boolean arguments expected".to_string()),
          }
        },
      }
    },
    // CONTROL
    "?" => {
      match expect_args(3, &params, &id) {
        Some(e) => e,
        None => {
          match params[0] {
            Evaluation::Exception(_) => params[0].clone(),
            Evaluation::True => params[1].clone(),
            Evaluation::False => params[2].clone(),
            _ => evaluator::exception(ExceptionType::TypeError, &id,
                                      "expected boolean for first argument".to_string()),
          }
        },
      }
    },
    "=" => {
      match expect_args(2, &params, &id) {
        Some(e) => e,
        None => {
          match params[0] {
            Evaluation::True => {
              match params[1] {
                Evaluation::True => Evaluation::True,
                _ => Evaluation::False,
              }
            },
            Evaluation::False => {
              match params[1] {
                Evaluation::False => Evaluation::True,
                _ => Evaluation::False,
              }
            },
            Evaluation::Integer(x) => {
              match params[1] {
                Evaluation::Integer(y) => {
                  if x == y {
                    Evaluation::True
                  } else {
                    Evaluation::False
                  }
                },
                _ => Evaluation::False,
              }
            },
            Evaluation::Float(x) => {
              match params[1] {
                Evaluation::Float(y) => {
                  if x == y {
                    Evaluation::True
                  } else {
                    Evaluation::False
                  }
                },
                _ => Evaluation::False,
              }
            },
            Evaluation::String(ref x) => {
              match params[1] {
                Evaluation::String(ref y) => {
                  if &x == &y {
                    Evaluation::True
                  } else {
                    Evaluation::False
                  }
                },
                _ => Evaluation::False,
              }
            },
            Evaluation::List(ref x) => {
              match params[1] {
                Evaluation::List(ref y) => {
                  if x.items.len() != y.items.len() {
                    Evaluation::False
                  } else {
                    let mut rc = Evaluation::True;
                    for n in 0..x.items.len() {
                      let cmp = vec![x.items[n].clone(), y.items[n].clone()];
                      match system_functions("=".to_string(), cmp) {
                        Evaluation::True => {
                          // do nothing, everything still matches
                        },
                        _ => {
                          rc = Evaluation::False;
                          break;
                        }
                      }
                    }
                    rc
                  }
                },
                _ => Evaluation::False,
              }
            },
            _ => Evaluation::False,
          }
        },
      }
    },
    ">" => {
      match expect_args(2, &params, &id) {
        Some(e) => e,
        None => {
          match params[0] {
            Evaluation::Integer(x) => {
              match params[1] {
                Evaluation::Integer(y) => {
                  if x > y {
                    Evaluation::True
                  } else {
                    Evaluation::False
                  }
                },
                Evaluation::Float(y) => {
                  if x as f64 > y {
                    Evaluation::True
                  } else {
                    Evaluation::False
                  }
                },
                _ => evaluator::exception(ExceptionType::TypeError, &id,
                                          "numeric arguments expected".to_string()),
              }
            },
            Evaluation::Float(x) => {
              match params[1] {
                Evaluation::Integer(y) => {
                  if x > y as f64 {
                    Evaluation::True
                  } else {
                    Evaluation::False
                  }
                },
                Evaluation::Float(y) => {
                  if x > y {
                    Evaluation::True
                  } else {
                    Evaluation::False
                  }
                },
                _ => evaluator::exception(ExceptionType::TypeError, &id,
                                          "numeric arguments expected".to_string()),
              }
            },
            _ => evaluator::exception(ExceptionType::TypeError, &id,
                                      "numeric arguments expected".to_string()),
          }
        },
      }
    },
    "<" => {
      match expect_args(2, &params, &id) {
        Some(e) => e,
        None => {
          match params[0] {
            Evaluation::Integer(x) => {
              match params[1] {
                Evaluation::Integer(y) => {
                  if x < y {
                    Evaluation::True
                  } else {
                    Evaluation::False
                  }
                },
                Evaluation::Float(y) => {
                  // weird parser bug seems to require parens here but not elsewhere
                  if (x as f64) < y {
                    Evaluation::True
                  } else {
                    Evaluation::False
                  }
                },
                _ => evaluator::exception(ExceptionType::TypeError, &id,
                                          "numeric arguments expected".to_string()),
              }
            },
            Evaluation::Float(x) => {
              match params[1] {
                Evaluation::Integer(y) => {
                  if x < y as f64 {
                    Evaluation::True
                  } else {
                    Evaluation::False
                  }
                },
                Evaluation::Float(y) => {
                  if x < y {
                    Evaluation::True
                  } else {
                    Evaluation::False
                  }
                },
                _ => evaluator::exception(ExceptionType::TypeError, &id,
                                          "numeric arguments expected".to_string()),
              }
            },
            _ => evaluator::exception(ExceptionType::TypeError, &id,
                                      "numeric arguments expected".to_string()),
          }
        },
      }
    },
    "substr" => {
      match expect_args(3, &params, &id) {
        Some(e) => e,
        None => {
          match params[0] {
            Evaluation::String(ref s) => {
              match params[1] {
                Evaluation::Integer(start) => {
                  match params[2] {
                    Evaluation::Integer(len) => {
                      let chars = s.chars();
                      if start as usize >= s.len() {
                        Evaluation::String("".to_string())
                      } else if (start + len) as usize >= s.len() {
                        let rc = chars.skip(start as usize).take(s.len() - start as usize).collect();
                        Evaluation::String(rc)
                      } else {
                        let rc = chars.skip(start as usize).take(len as usize).collect();
                        Evaluation::String(rc)
                      }
                    },
                    _ => evaluator::exception(ExceptionType::TypeError, &id,
                                              "third argument expects integer for length".to_string()),
                  }
                },
                _ => evaluator::exception(ExceptionType::TypeError, &id,
                                          "second argument expects integer for start".to_string()),
              }
            },
            _ => evaluator::exception(ExceptionType::TypeError, &id,
                                      "first argument must be string".to_string()),
          }
        },
      }
    },
    "strlen" => {
      match expect_args(1, &params, &id) {
        Some(e) => e,
        None => {
          match params[0] {
            Evaluation::String(ref s) => {
              Evaluation::Integer(s.chars().count() as i64)
            },
            _ => evaluator::exception(ExceptionType::TypeError, &id,
                                      "string argument expected".to_string()),
          }
        },
      }
    },
    "car" => {
      match expect_args(1, &params, &id) {
        Some(e) => e,
        None => {
          match params[0] {
            Evaluation::List(ref list) => {
              match list.items.first() {
                Some(item) => {
                  item.clone()
                },
                _ => evaluator::exception(ExceptionType::RuntimeError, &id,
                                          "attempt to get first item of empty list".to_string()),
              }
            },
            _ => evaluator::exception(ExceptionType::TypeError, &id,
                                      "list argument expected".to_string()),
          }
        },
      }
    },
    "cdr" => {
      match expect_args(1, &params, &id) {
        Some(e) => e,
        None => {
          match params[0] {
            Evaluation::List(ref list) => {
              match list.tail {
                Some(tail) => tail.clone(),
                None => list.clone(),
              }
            },
            _ => evaluator::exception(ExceptionType::TypeError, &id,
                                      "list argument expected".to_string()),
          }
        },
      }
    },
    "catch" => {
      match expect_args(1, &params, &id) {
        Some(e) => e,
        None => {
          match params[0] {
            Evaluation::Exception(ref e) => Evaluation::List(e.to_list()),
            ref eval => Evaluation::List(ListEval {
              head: Some(Box::new(Evaluation::String("ok".to_string()))),
              tail: Some(Box::new(ListEval {
                head: Some(Box::new(eval.clone())),
                tail: Some(Box::new(ListEval { head: None, tail: None }))
              }))
            }),
          }
        },
      }
    },
    "raise" => {
      match expect_args(1, &params, &id) {
        Some(e) => e,
        None => {
          Evaluation::Exception(Exception { flavor: ExceptionType::Error,
                                            payload: Box::new(params[0].clone()),
                                            stack: Vec::new() })
        },
      }
    },
    "~" => {
      match expect_args(1, &params, &id) {
        Some(e) => e,
        None => {
          Evaluation::Exception(Exception { flavor: ExceptionType::Return,
                                            payload: Box::new(params[0].clone()),
                                            stack: Vec::new() })
        },
      }
    },
    _ => evaluator::exception(ExceptionType::UndefError, &id,
                              "function is not defined in scope".to_string()),
  }
}
