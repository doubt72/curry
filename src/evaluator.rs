// Evaluate parsed stuff

use encoding::Block;
use encoding::Evaluation;
use encoding::Exception;
use encoding::ExceptionType;

// Look how simple this is!  ...Because we hid all of the logic in the types

pub fn exception(flavor: ExceptionType, id: &String, msg: String) ->
  Evaluation {
  Evaluation::Exception(Exception::new(&flavor,
                        &Evaluation::String(format!("{} : {}", id, msg))))
}

pub fn evaluate(block: &Block) {
  let mut scope = Vec::new();
  let result = block.evaluate(&mut scope, &"[main program]".to_string());
  match &result {
    &Evaluation::Exception(ref e) => {
      println!("{}", e);
    },
    _ => {
      // not an error, do nothing
    },
  }
}
