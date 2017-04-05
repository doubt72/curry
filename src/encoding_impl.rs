use std::collections::HashMap;

use evaluator;
use primitives;

use encoding::Block;
use encoding::Expression;
use encoding::List;
use encoding::Call;
use encoding::Definition;

use encoding::Scope;
use encoding::FunctionOrValue;
use encoding::Evaluation;
use encoding::ListEval;
use encoding::Function;
use encoding::Exception;
use encoding::ExceptionType;

impl List {
  pub fn evaluate(&self, scope: &mut Vec<Scope>) -> Evaluation {
    match self.head {
      Some(Expression(ref e)) => {
        match self.tail {
          Some(List(ref list)) =>  {
            Evaluation::List(
              ListEval { head: Some(Box::new(e.evaluate(scope))),
                         tail: Some(Box::new(list.evaluate(scope))) }
            )
          },
          None => {
            // This should never happen
            Evaluation::List(ListEval { head: None, tail: None })
          },
        }
      },
      None => Evaluation::List(ListEval { head: None, tail: None }),
    }
  }

  pub fn clone(&self) -> List {
    match self.head {
      Some(Expression(ref e)) => {
        match self.tail {
          Some(List(ref list)) =>  {
            List { head: Some(Box::new(e.clone())),
                   tail: Some(Box::new(list.clone())) }
          },
          None => {
            // This should never happen
            List { head: None, tail: None }
          },
        }
      },
      None => List { head: None, tail: None},
    }
  }
}

impl Call {
  pub fn evaluate(&self, scope: &mut Vec<Scope>) -> Evaluation {
    let mut check = false;
    let rc;

    // Debug here:
    //println!("CALLING {} IN", &self.id);
    //for n in 0..scope.len() {
    //  println!("  {:?}", scope[n]);
    //}

    // Search through scopes in reverse order for function (or value)
    let mut binding = FunctionOrValue::Value(Evaluation::False);
    for x in (0..scope.len()).rev() {
      if scope[x].bindings.contains_key(&self.id) {
        check = true;
        binding = scope[x].bindings[&self.id].clone();
        break;
      }
    }
    if check {
      match binding {
        FunctionOrValue::Function(ref func) => {
          // Add a scope for the function parameters, evaluate, and populate
          let mut p_scope = Scope { bindings: HashMap::new() };
          if self.params.len() != func.params.len() {
            return evaluator::exception(ExceptionType::ArgError, &self.id,
                                        format!("expected argument list of length {} but got {}",
                                                self.params.len(),
                                                func.params.len()));
          }
          for y in 0..self.params.len() {
            let mut block = Block { expressions: Vec::new() };
            block.expressions.push(self.params[y].clone());
            let eval = block.evaluate(scope, &self.id);
            p_scope.bindings.insert(func.params[y].clone(),
                                    FunctionOrValue::Value(eval));
          }
          scope.push(p_scope);
          rc = func.block.evaluate(scope, &self.id);
          scope.pop();
        },
        FunctionOrValue::Value(ref value) => {
          // This value has already been evaluated, i.e., it's a passed param
          rc = value.clone();
        },
      }
    } else if self.id == "$" {
      if self.params.len() < 1 {
        rc = evaluator::exception(ExceptionType::ArgError, &self.id,
                                  "expected argument list of at least length 1 but got 0".to_string());
      } else {
        let eval = self.params[0].evaluate(scope);
        match eval {
          Evaluation::Exception(_) => { return eval.clone(); },
          Evaluation::Function(ref func) => {
            // TODO: make helper function
            // This is (almost) repeated for user functions (minus first param)
            let mut p_scope = Scope { bindings: HashMap::new() };
            if self.params.len() - 1 != func.params.len() {
              return evaluator::exception(ExceptionType::ArgError, &self.id,
                                          format!("called function expected argument list of length {} but got {}",
                                                  self.params.len() - 1,
                                                  func.params.len()));
            }
            for y in 1..self.params.len() {
              let mut block = Block { expressions: Vec::new() };
              block.expressions.push(self.params[y].clone());
              let eval = block.evaluate(scope, &self.id);
              p_scope.bindings.insert(func.params[y-1].clone(),
                                      FunctionOrValue::Value(eval));
            }
            scope.push(p_scope);
            rc = func.block.evaluate(scope, &self.id);
            scope.pop();
          },
          _ => {
            rc = evaluator::exception(ExceptionType::TypeError, &self.id,
                                      "function expected as first argument".to_string());
          },
        }
      }
    } else {
      // Try low-level system functions
      let mut params = Vec::new();
      for p in &self.params {
        let eval = p.evaluate(scope);
        params.push(eval);
      }
      rc = primitives::system_functions(self.id.clone(), params);
    }
    rc
  }

  pub fn clone(&self) -> Call {
    let mut call = Call { id: self.id.clone(), params: Vec::new() };
    for p in &self.params {
      call.params.push(p.clone());
    }
    call
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
      let mut func = Function { params: Vec::new(), block: self.block.clone() };
      for p in &self.params {
        func.params.push(p.clone());
      }
      s.bindings.insert(self.id.clone(),
                         FunctionOrValue::Function(func.clone()));
      Evaluation::Function(func)
    } else {
      panic!("internal error: no scope supplied to definition evaluation");
    }
  }

  pub fn clone(&self) -> Definition {
    let mut def = Definition { id: self.id.clone(), params: Vec::new(),
                               block: self.block.clone() };
    for p in &self.params {
      def.params.push(p.clone());
    }
    def
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
      &Expression::List(ref list) => list.evaluate(scope),
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
  pub fn evaluate(&self, scope: &mut Vec<Scope>, context: &String) ->
    Evaluation {

    // Add current context
    let current = Scope { bindings: HashMap::new() };
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
      // Debug output:
      //println!("{:?}", value);
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

impl FunctionOrValue {
  pub fn clone(&self) -> FunctionOrValue {
    match self {
      &FunctionOrValue::Function(ref func) => {
        FunctionOrValue::Function(func.clone())
      },
      &FunctionOrValue::Value(ref value) => {
        FunctionOrValue::Value(value.clone())
      },
    }
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
    match self.head {
      Some(Evaluation(ref e)) => {
        match self.tail {
          Some(List(ref list)) =>  {
            ListEval { head: Some(Box::new(e.clone())),
                       tail: Some(Box::new(list.clone()))}
          },
          None => {
            // This should never happen
            ListEval { head: None, tail: None }
          },
        }
      },
      None => ListEval { head: None, tail: None},
    }
  }
}

impl Function {
  pub fn clone(&self) -> Function {
    let mut func = Function { params: Vec::new(), block: self.block.clone() };
    for p in &self.params {
      func.params.push(p.clone());
    }
    func
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
    let mut stack = ListEval { head: None, tail: None };
    for i in (0..self.stack.len()).rev() {
      stack = ListEval {
        head: Some(Box::new(Evaluation::String(self.stack[i].clone()))),
        tail: Some(Box::new(stack))
      };
    }
    ListEval {
      head: Some(Box::new(Evaluation::String(e.flavor.to_string()))),
      tail: Some(Box::new(ListEval {
        head: Some(Box::new(e.payload.clone())),
        tail: Some(Box::new(ListEval {
          head: Some(Box::new(Evaluation::List(stack))),
          tail: Some(Box::new(ListEval { head: None, tail: None }))
        }))
      }))
    }
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
