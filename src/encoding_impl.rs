use std::collections::HashMap;

use evaluator;
use primitives;

use encoding::Block;
use encoding::Expression;
use encoding::List;
use encoding::Call;
use encoding::Definition;

use encoding::Scope;
use encoding::Evaluation;
use encoding::ListEval;
use encoding::Function;
use encoding::Exception;
use encoding::ExceptionType;

impl List {
  pub fn evaluate(&self, scope: &mut Vec<Scope>) -> ListEval {
    let mut list = ListEval { items: Vec::new() };
    for i in &self.items {
      list.items.push(i.evaluate(scope));
    }
    list
  }

  pub fn clone(&self) -> List {
    let mut list = List { items: Vec::new() };
    for i in &self.items {
      list.items.push(i.clone());
    }
    list
  }
}

impl Call {
  pub fn evaluate(&self, scope: &mut Vec<Scope>) -> Evaluation {
    for x in (0..scope.len()).rev() {
      if scope[x].bindings.contains_key(&self.id) {
        let binding = scope[x].bindings[&self.id].clone();
        let eval = self.param.evaluate(scope);
        return binding.block.evaluate(scope, &eval, &self.id);
      }
    }

    if self.id == "," {
      if self.param.items.len() < 2 {
        return evaluator::exception(ExceptionType::ArgError, &self.id,
          format!("expected argument list of length 2 but got {}",
                  self.param.items.len()));
      }
      let eval = self.param.items[0].evaluate(scope);
      match eval {
        Evaluation::Exception(_) => { return eval; },
        Evaluation::Function(ref func) => {
          match self.param.items[1] {
            Expression::List(ref list) => {
              let elist = list.evaluate(scope);
              return func.block.evaluate(scope, &elist, &self.id);
            },
            _ => {
              return evaluator::exception(ExceptionType::TypeError, &self.id,
                "list expected as second argument".to_string());
            },
          }
        }
        _ => {
          return evaluator::exception(ExceptionType::TypeError, &self.id,
            "function expected as first argument".to_string());
        },
      }
    } else {
      match self.id.chars().nth(0) {
        Some('_') => {
          // Parametric functions
          let mut depth = 0;
          for c in self.id.chars() {
            if c == '_' {
              depth += 1;
            } else {
              return evaluator::exception(ExceptionType::TypeError, &self.id,
                "function is not defined in scope".to_string());
            }
          }
          if depth > scope.len() {
            return evaluator::exception(ExceptionType::TypeError, &self.id,
              "attempt to reach out of main scope".to_string());
          }
          let param = scope[scope.len() - depth].param.clone();
          Evaluation::List(param)
        },
        _ => {
          // Try low-level system functions
          primitives::system_functions(self.id.clone(),
                                       self.param.evaluate(scope))
        }
      }
    }
  }

  pub fn clone(&self) -> Call {
    Call { id: self.id.clone(), param: self.param.clone() }
  }
}

impl Definition {
  pub fn evaluate(&self, scope: &mut Vec<Scope>) -> Evaluation {
    let last = scope.last_mut();
    if let Some(s) = last {
      if s.bindings.contains_key(&self.id) {
        return evaluator::exception(ExceptionType::RedefError, &"".to_string(),
                                    format!("attempt to redefine {}",
                                            self.id));
      }
      let func = Function { block: self.block.clone() };
      s.bindings.insert(self.id.clone(), func.clone());
      Evaluation::Function(func)
    } else {
      panic!("internal error: no scope supplied to definition evaluation");
    }
  }

  pub fn clone(&self) -> Definition {
    Definition { id: self.id.clone(), block: self.block.clone() }
  }
}

impl Expression {
  pub fn evaluate(&self, scope: &mut Vec<Scope>) -> Evaluation {
    match self {
      &Expression::True => Evaluation::True,
      &Expression::False => Evaluation::False,
      &Expression::Integer(x) => Evaluation::Integer(x),
      &Expression::Float(x) => Evaluation::Float(x),
      &Expression::String(ref s) => Evaluation::String(s.clone()),
      &Expression::List(ref list) => Evaluation::List(list.evaluate(scope)),
      &Expression::Call(ref call) => call.evaluate(scope),
      &Expression::Definition(ref def) => def.evaluate(scope),
    }
  }

  pub fn clone(&self) -> Expression {
    match self {
      &Expression::True => Expression::True,
      &Expression::False => Expression::False,
      &Expression::Integer(x) => Expression::Integer(x),
      &Expression::Float(x) => Expression::Float(x),
      &Expression::String(ref s) => Expression::String(s.clone()),
      &Expression::List(ref list) => Expression::List(list.clone()),
      &Expression::Call(ref call) => Expression::Call(call.clone()),
      &Expression::Definition(ref def) => Expression::Definition(def.clone()),
    }
  }
}

impl Block {
  pub fn evaluate(&self, scope: &mut Vec<Scope>, param: &ListEval,
                  context: &String) ->
    Evaluation {

    // Add current context
    let current = Scope { bindings: HashMap::new(), param: param.clone() };
    scope.push(current);

    // Evaluate
    let mut value = Evaluation::False;
    for e in &self.expressions {
      match e.evaluate(scope) {
        Evaluation::Exception(ref ex) => {
          match &ex.flavor {
            &ExceptionType::Return => { return ex.payload.clone(); },
            _ => {
              let mut rc = ex.clone();
              rc.stack.push(context.clone());
              return Evaluation::Exception(rc);
            },
          }
        },
        ev => { value = ev },
      }
    }
    // Current context going out of scope
    scope.pop();
    value
  }

  pub fn clone(&self) -> Block {
    let mut rc = Block { expressions: Vec::new() };
    for i in &self.expressions {
      rc.expressions.push(i.clone());
    }
    rc
  }
}

impl Evaluation {
  pub fn clone(&self) -> Evaluation {
    match self {
      &Evaluation::True => Evaluation::True,
      &Evaluation::False => Evaluation::False,
      &Evaluation::Integer(x) => Evaluation::Integer(x),
      &Evaluation::Float(x) => Evaluation::Float(x),
      &Evaluation::String(ref s) => Evaluation::String(s.clone()),
      &Evaluation::List(ref list) => Evaluation::List(list.clone()),
      &Evaluation::Exception(ref e) => Evaluation::Exception(e.clone()),
      &Evaluation::Function(ref func) => Evaluation::Function(func.clone()),
    }
  }
}

impl ListEval {
  pub fn clone(&self) -> ListEval {
    let mut list = ListEval { items: Vec::new() };
    for i in &self.items {
      list.items.push(i.clone());
    }
    list
  }
}

impl Function {
  pub fn clone(&self) -> Function {
    Function { block: self.block.clone() }
  }
}

impl Exception {
  pub fn new(flavor: &ExceptionType, payload: &Evaluation) -> Exception {
    Exception {
      flavor: flavor.clone(),
      payload: Box::new(payload.clone()),
      stack: Vec::new()
    }
  }

  pub fn clone(&self) -> Exception {
    let mut e = Exception::new(&self.flavor, &*self.payload);
    for i in &self.stack {
      e.stack.push(i.clone());
    }
    e
  }

  pub fn to_list(&self) -> ListEval {
    let mut stack = ListEval { items: Vec::new() };
    let mut rc = ListEval { items: Vec::new() };
    rc.items.push(Evaluation::String(self.flavor.to_string()));
    rc.items.push(self.payload.clone());
    for i in &self.stack {
      stack.items.push(Evaluation::String(i.clone()));
    }
    rc.items.push(Evaluation::List(stack));
    rc
  }
}

impl ExceptionType {
  pub fn clone(&self) -> ExceptionType {
    match self {
      &ExceptionType::Return => ExceptionType::Return,
      &ExceptionType::Error => ExceptionType::Error,
      &ExceptionType::ArgError => ExceptionType::ArgError,
      &ExceptionType::ParseError => ExceptionType::ParseError,
      &ExceptionType::TypeError => ExceptionType::TypeError,
      &ExceptionType::TypeMismatch => ExceptionType::TypeMismatch,
      &ExceptionType::DivByZero => ExceptionType::DivByZero,
      &ExceptionType::RuntimeError => ExceptionType::RuntimeError,
      &ExceptionType::UndefError => ExceptionType::UndefError,
      &ExceptionType::RedefError => ExceptionType::RedefError,
    }
  }
}
