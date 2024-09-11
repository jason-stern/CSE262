use crate::parser::Node;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
  String(String),
  Number(i32),
  Bool(bool),
}


pub struct Runtime {
  functions: HashMap<String, Vec<Node>>,
  stack: Vec<HashMap<String, Value>>,
}

impl Runtime {

  pub fn new() -> Runtime {
    Runtime {
      functions: HashMap::new(),
      stack: Vec::new(),
    }
  }

  // Define the `run` method of the `Runtime` struct.
  pub fn run(&mut self, node: &Node) -> Result<Value, &'static str> {
    // Match the type of the input `Node`.
    match node {
        // If the `Node` is a `Program`, evaluate each of its children in sequence.
        Node::Program { children } => {
            for n in children {
                match n {
                    // If the child node is a `FunctionDefine`, add it to the list of functions.
                    Node::FunctionDefine { .. } => {
                        let result = self.run(n);
                        match &result {
                            Err(_) => {
                                return result;
                            }
                            _ => (),
                        }
                    },
                    // If the child node is an `Expression`, add it as the body of a new `main` function.
                    Node::Expression { .. } => {
                        self.functions.insert("main".to_string(), vec![Node::FunctionReturn { children: vec![n.clone()] }]);
                    },
                    // If the child node is a `Statement`, add it as the body of a new `main` function.
                    Node::Statement { .. } => {
                        self.functions.insert("main".to_string(), vec![n.clone()]);
                    }
                    Node::IfExpression { .. } => {
                        let result = self.run(n);
                        match &result {
                            Err(_) => {
                                return result;
                            }
                            _ => (),
                        }
                    }
                    // Ignore any other type of child node.
                    _ => (),
                }
            }
            // Return `Value::Bool(true)` wrapped in a `Result`.
            Ok(Value::Bool(true))
        },
        // If the `Node` is a `MathExpression`, evaluate it.
        Node::MathExpression { name, children } => {
            // Evaluate the left and right children of the `MathExpression`.
            match (self.run(&children[0]), self.run(&children[1])) {
                // If both children are `Number` values, extract their values and evaluate the expression.
                (Ok(Value::Number(lhs)), Ok(Value::Number(rhs))) => {
                    match name.as_ref() {
                        // If the operator is `+`, add the values.
                        "+" => Ok(Value::Number(lhs + rhs)),
                        // If the operator is `-`, subtract the values.
                        "-" => Ok(Value::Number(lhs - rhs)),
                        // If the operator is `*`, multiply the values.
                        "*" => Ok(Value::Number(lhs * rhs)),
                        // If the operator is `/`, divide the values.
                        "/" => Ok(Value::Number(lhs / rhs)),
                        // If the operator is `^`, raise the left value to the power of the right value.
                        "^" => {
                            let mut result = 1;
                            for _i in 0..rhs {
                                result = result * lhs;
                            }
                            Ok(Value::Number(result))
                        },
                        // If the operator is not recognized, return an error message.
                        _ => Err("Undefined operator"),
                    }
                }
                // If either child is not a `Number` value, return an error message.
                _ => Err("Cannot do math on String or Bool"),
            }
        },
        // If the `Node` is a `FunctionCall`, evaluate it.
        Node::FunctionCall { name, children } => {
            // Extract the input arguments.
            let in_args = if children.len() > 0 {
                match &children[0] {
                    Node::FunctionArguments { children } => {
                        children
                    },
                    _ => children,
                }
            } else {
                children
            };
            // Create a new frame for local variables.
            let mut new_frame = HashMap::new();
            // Initialize the result to an error message.
            let mut result: Result<Value, &'static str> = Err("Undefined function");
            // Save a raw pointer to the `Runtime` instance for use in the nested closure.
            let rt = self as *mut Runtime;
            // Find the named function and evaluate its body.
            match self.functions.get(name) {
                Some(statements) => {
                    {
                        // If the function has input arguments, bind their values to the corresponding parameters.
                        match statements[0].clone() {
                            Node::FunctionArguments { children } => {
                                for (ix, arg) in children.iter().enumerate() {
                                    // Use unsafe Rust code to call `run` on the input argument and handle any errors.
                                    unsafe {
                                        let result = (*rt).run(&in_args[ix])?;
                                        match arg {
                                            Node::Expression { children } => {
                                                match &children[0] {
                                                    Node::Identifier { value } => {
                                                        new_frame.insert(value.clone(), result);
                                                    },
                                                    _ => (),
                                                }
                                            }
                                            _ => (),
                                        }
                                    }
                                }
                            }
                            _ => (),
                        }
                    }
                    // Push the new frame onto the stack.
                    self.stack.push(new_frame);
                    // Evaluate each statement in the function body.
                    for n in statements.clone() {
                        result = self.run(&n);
                    }
                    // Pop the frame off the stack.
                    self.stack.pop();
                },
                None => (),
            };
            // Return the result of evaluating the function.
            result
        },
        // If the `Node` is a `FunctionDefine`, add it to the list of functions.
        Node::FunctionDefine { children } => {
            let (head, tail) = children.split_at(1);
            match &head[0] {
                Node::Identifier { value } => {
                    self.functions.insert(value.to_string(), tail.to_vec());
                },
                _ => (),
            }
            Ok(Value::Bool(true))
        },
        // If the `Node` is a `FunctionReturn`, evaluate its child node.
        Node::FunctionReturn { children } => {
            self.run(&children[0])
        },
        // If the `Node` is an `Identifier`, look up its value in the current frame.
        Node::Identifier { value } => {
            let last = self.stack.len() - 1;
            match self.stack[last].get(value) {
                Some(id_value) => Ok(id_value.clone()),
                None => Err("Undefined variable"),
            }
        },
        // Final exam - If Expression
        Node::IfExpression { children} => {
            for c in children {
                let result = self.run(c);
                match result {
                    Ok(Value::Bool(true)) => {
                        return result;
                    }
                    Ok(Value::Bool(false)) => (),
                    _ => return result,
                }
            }
            Ok(Value::Bool(true))
        },
        // Evaluate it's condition; if true, run the statements
        Node::IfBlock { condition, children } => {
            let result = self.run(&condition[0]);
            match result {
                Ok(Value::Bool(true)) => {
                    for c in children {
                        match c {
                            Node::Statement{ children: func_return } => {
                                match func_return[0] {
                                    Node::FunctionReturn{..} => {
                                        return self.run(&func_return[0]);
                                    }
                                    _ => (),
                                }
                            }
                            _ => (),
                        }
                        let result_2 = self.run(c);
                        match &result_2 {
                            Err(_) => {
                                return result_2;
                            }
                            _ => (),
                        }
                    }
                    return Ok(Value::Bool(true));
                }
                Ok(Value::Bool(false)) => (),
                _ => {
                    return result;
                }
            }
            return Ok(Value::Bool(false));
        },
        // Evaluate it's condition; if true, run the statements
        Node::ElseIfBlock { condition, children } => {
            match self.run(&condition[0]) {
                Ok(Value::Bool(true)) => {
                    for c in children {
                        match c {
                            Node::Statement{ children: func_return } => {
                                match func_return[0] {
                                    Node::FunctionReturn{..} => {
                                        return self.run(&func_return[0]);
                                    }
                                    _ => (),
                                }
                            }
                            _ => (),
                        }
                        let result = self.run(c);
                        match &result {
                            Err(_) => {
                                return result;
                            }
                            _ => (),
                        }
                    }
                    return Ok(Value::Bool(true));
                }
                Ok(Value::Bool(false)) => (),
                _ => {
                    return Err("Not a boolean value");
                }
            }
            return Ok(Value::Bool(false));
        },
        // No condition; run statements
        Node::ElseBlock { children } => {
            for c in children {
                match c {
                    Node::Statement{ children: func_return } => {
                        match func_return[0] {
                            Node::FunctionReturn{..} => {
                                return self.run(&func_return[0]);
                            }
                            _ => (),
                        }
                    }
                    _ => (),
                }
                let result = self.run(c);
                match &result {
                    Err(_) => {
                        return result;
                    }
                    _ => (),
                }
            }
            Ok(Value::Bool(true))
        },
        // If the `Node` is a `Statement`, evaluate its child node.
        Node::Statement { children } => {
            match &children[0] {
                Node::VariableDefine { .. } |
                Node::FunctionReturn { .. } | 
                Node::FunctionCall {..} => { self.run(&children[0]) },
                _ => Err("Unknown Statement"),
            }
        },
        // If the `Node` is a `VariableDefine`, evaluate its expression and bind the result to a new variable.
        Node::VariableDefine { children } => {
            // Extract the variable name.
            let name: String = match &children[0] {
                Node::Identifier { value } => value.clone(),
                _ => "".to_string(),
            };
            // Evaluate the expression.
            let value = self.run(&children[1])?;
            // Add the variable to the current frame.
            let last = self.stack.len() - 1;
            self.stack[last].insert(name, value.clone());
            // Return the value.
            Ok(value)
        },
        // If the `Node` is an `Expression`, evaluate its child node.
        Node::Expression { children } => {
            match &children[0] {
                Node::MathExpression { .. } |
                Node::Number { .. } |
                Node::FunctionCall { .. } |
                Node::String { .. } |
                Node::Bool { .. } |
                Node::Identifier { .. } => {
                    self.run(&children[0])
                },
                Node::ComparisonOperator { operator, children } => { // Handles comparison operators in VarDefs.
                    match operator.as_ref() {
                        "==" => {
                            let node_1 = &children[0];
                            let node_2 = &children[1];
                            match &node_1 {
                                Node::Number{..} | Node::MathExpression{..} | Node::Expression{..} => {
                                    match &node_2 {
                                        Node::Number{..} | Node::Identifier {..} | Node::MathExpression {..} => {
                                            Ok(Value::Bool(self.run(&node_1) == self.run(&node_2)))
                                        },
                                        _ => {Err("Invalid expression - can only compare numbers to numbers")}
                                    }
                                }
                                _ => {
                                    Err("Invalid expression")
                                }
                            }
                        },
                        "!=" => {
                            let node_1 = &children[0];
                            let node_2 = &children[1];
                            match &node_1 {
                                Node::Number{..} | Node::MathExpression{..} | Node::Expression{..} => {
                                    match &node_2 {
                                        Node::Number{..} | Node::Identifier {..} | Node::MathExpression {..} => {
                                            Ok(Value::Bool(self.run(&node_1) != self.run(&node_2)))
                                        },
                                        _ => {Err("Invalid expression - can only compare numbers to numbers")}
                                    }
                                }
                                _ => {
                                    Err("Invalid expression")
                                }
                            }
                        },
                        ">" => {
                            let node_1 = &children[0];
                            let node_2 = &children[1];
                            match &node_1 {
                                Node::Number{..} | Node::MathExpression{..} | Node::Expression{..} => {
                                    match &node_2 {
                                        Node::Number{..} | Node::Identifier {..} | Node::MathExpression {..} => {
                                            match (self.run(&node_1), self.run(&node_2)) { // Gets the i32 values from the Number nodes
                                                (Ok(Value::Number(val_1)), Ok(Value::Number(val_2))) => {
                                                    Ok(Value::Bool(val_1 > val_2))
                                                }
                                                _ => Err("Unsuccesful interpreting > comparison"),
                                            }
                                        },
                                        _ => {Err("Invalid expression - can only compare numbers to numbers")}
                                    }
                                }
                                _ => {
                                    Err("Invalid expression")
                                }
                            }
                        },
                        "<" => {
                            let node_1 = &children[0];
                            let node_2 = &children[1];
                            match &node_1 {
                                Node::Number{..} | Node::MathExpression{..} | Node::Expression{..} => {
                                    match &node_2 {
                                        Node::Number{..} | Node::Identifier {..} | Node::MathExpression {..} => {
                                            match (self.run(&node_1), self.run(&node_2)) { // Gets the i32 values from the Number nodes
                                                (Ok(Value::Number(val_1)), Ok(Value::Number(val_2))) => {
                                                    Ok(Value::Bool(val_1 < val_2))
                                                }
                                                _ => Err("Unsuccesful interpreting < comparison"),
                                            }
                                        },
                                        _ => {Err("Invalid expression - can only compare numbers to numbers")}
                                    }
                                }
                                _ => {
                                    Err("Invalid expression3")
                                }
                            }
                        },
                        ">=" => {
                            let node_1 = &children[0];
                            let node_2 = &children[1];
                            match &node_1 {
                                Node::Number{..} | Node::MathExpression{..} | Node::Expression{..} => {
                                    match &node_2 {
                                        Node::Number{..} | Node::Identifier {..} | Node::MathExpression {..} => {
                                            match (self.run(&node_1), self.run(&node_2)) { // Gets the i32 values from the Number nodes
                                                (Ok(Value::Number(val_1)), Ok(Value::Number(val_2))) => {
                                                    Ok(Value::Bool(val_1 >= val_2))
                                                }
                                                _ => Err("Unsuccesful interpreting >= comparison"),
                                            }
                                        },
                                        _ => {Err("Invalid expression - can only compare numbers to numbers")}
                                    }
                                }
                                _ => {
                                    Err("Invalid expression")
                                }
                            }
                        }
                        "<=" => {
                            let node_1 = &children[0];
                            let node_2 = &children[1];
                            match &node_1 {
                                Node::Number{..} | Node::MathExpression{..} | Node::Expression{..} | Node::Identifier{..} => {
                                    match &node_2 {
                                        Node::Number{..} | Node::Identifier {..} | Node::MathExpression {..} => {
                                            match (self.run(&node_1), self.run(&node_2)) { // Gets the i32 values from the Number nodes
                                                (Ok(Value::Number(val_1)), Ok(Value::Number(val_2))) => {
                                                    Ok(Value::Bool(val_1 <= val_2))
                                                }
                                                _ => Err("Unsuccesful interpreting <= comparison"),
                                            }
                                        },
                                        _ => {Err("Invalid expression - can only compare numbers to numbers")}
                                    }
                                }
                                _ => {
                                    Err("Invalid expression")
                                }
                            }
                        }
                        _ => {
                            return Err("Invalid operator");
                        }
                    }
                }
                _ => Err("Unknown Expression"),
            }
        },
        Node::ComparisonOperator { operator, children } => { // Handles comparison operators in if statements
            match operator.as_ref() {
                "==" => {
                    let node_1 = &children[0];
                    let node_2 = &children[1];
                    match &node_1 {
                        Node::Number{..} | Node::MathExpression{..} | Node::Expression{..} | Node::Identifier{..} => {
                            match &node_2 {
                                Node::Bool{..} | Node::Number{..} | Node::Identifier {..} | Node::MathExpression {..} => {
                                    Ok(Value::Bool(self.run(&node_1) == self.run(&node_2)))
                                },
                                _ => {Err("Invalid expression - can only compare numbers to numbers")}
                            }
                        }
                        _ => {
                            Err("Invalid expression")
                        }
                    }
                },
                "!=" => {
                    let node_1 = &children[0];
                    let node_2 = &children[1];
                    match &node_1 {
                        Node::Number{..} | Node::MathExpression{..} | Node::Expression{..} => {
                            match &node_2 {
                                Node::Bool{..} | Node::Number{..} | Node::Identifier {..} | Node::MathExpression {..} => {
                                    Ok(Value::Bool(self.run(&node_1) != self.run(&node_2)))
                                },
                                _ => {Err("Invalid expression - can only compare numbers to numbers")}
                            }
                        }
                        _ => {
                            Err("Invalid expression3")
                        }
                    }
                },
                ">" => {
                    let node_1 = &children[0];
                    let node_2 = &children[1];
                    match &node_1 {
                        Node::Number{..} | Node::MathExpression{..} | Node::Expression{..} => {
                            match &node_2 {
                                Node::Number{..} | Node::Identifier {..} | Node::MathExpression {..} => {
                                    match (self.run(&node_1), self.run(&node_2)) { // Gets the i32 values from the Number nodes
                                        (Ok(Value::Number(val_1)), Ok(Value::Number(val_2))) => {
                                            Ok(Value::Bool(val_1 > val_2))
                                        }
                                        _ => Err("Unsuccesful interpreting > comparison"),
                                    }
                                },
                                _ => {Err("Invalid expression - can only compare numbers to numbers")}
                            }
                        }
                        _ => {
                            Err("Invalid expression3")
                        }
                    }
                },
                "<" => {
                    let node_1 = &children[0];
                    let node_2 = &children[1];
                    match &node_1 {
                        Node::Number{..} | Node::MathExpression{..} | Node::Expression{..} => {
                            match &node_2 {
                                Node::Number{..} | Node::Identifier {..} | Node::MathExpression {..} => {
                                    match (self.run(&node_1), self.run(&node_2)) { // Gets the i32 values from the Number nodes
                                        (Ok(Value::Number(val_1)), Ok(Value::Number(val_2))) => {
                                            Ok(Value::Bool(val_1 < val_2))
                                        }
                                        _ => Err("Unsuccesful interpreting < comparison"),
                                    }
                                },
                                _ => {Err("Invalid expression - can only compare numbers to numbers")}
                            }
                        }
                        _ => {
                            Err("Invalid expression")
                        }
                    }
                },
                ">=" => {
                    let node_1 = &children[0];
                    let node_2 = &children[1];
                    match &node_1 {
                        Node::Number{..} | Node::MathExpression{..} | Node::Expression{..} => {
                            match &node_2 {
                                Node::Number{..} | Node::Identifier {..} | Node::MathExpression {..} => {
                                    match (self.run(&node_1), self.run(&node_2)) { // Gets the i32 values from the Number nodes
                                        (Ok(Value::Number(val_1)), Ok(Value::Number(val_2))) => {
                                            Ok(Value::Bool(val_1 >= val_2))
                                        }
                                        _ => Err("Unsuccesful interpreting >= comparison"),
                                    }
                                },
                                _ => {Err("Invalid expression - can only compare numbers to numbers")}
                            }
                        }
                        _ => {
                            Err("Invalid expression")
                        }
                    }
                }
                "<=" => {
                    let node_1 = &children[0];
                    let node_2 = &children[1];
                    match &node_1 {
                        Node::Number{..} | Node::MathExpression{..} | Node::Expression{..} => {
                            match &node_2 {
                                Node::Number{..} | Node::Identifier {..} | Node::MathExpression {..} => {
                                    match (self.run(&node_1), self.run(&node_2)) { // Gets the i32 values from the Number nodes
                                        (Ok(Value::Number(val_1)), Ok(Value::Number(val_2))) => {
                                            Ok(Value::Bool(val_1 <= val_2))
                                        }
                                        _ => Err("Unsuccesful interpreting <= comparison"),
                                    }
                                },
                                _ => {Err("Invalid expression - can only compare numbers to numbers")}
                            }
                        }
                        _ => {
                            Err("Invalid expression")
                        }
                    }
                }
                _ => {
                    return Err("Invalid operator");
                }
            }
        },
        // If the `Node` is a `Number`, wrap its value in a `Value::Number` and return it.
        Node::Number { value } => {
            Ok(Value::Number(*value))
        },
        // If the `Node` is a `String`, wrap its value in a `Value::String` and return it.
        Node::String { value } => {
            Ok(Value::String(value.clone()))
        },
        // If the `Node` is a `Bool`, wrap its value in a `Value::Bool` and return it.
        Node::Bool { value } => {
            Ok(Value::Bool(*value))
        },
        // If the `Node` is of an unhandled type, return an error message.
        _ => {
            Err("Unhandled Node")
        },
    }
  }
}

pub fn start_interpreter(node: &Node) -> Result<Value, &'static str> {
  let mut runtime = Runtime::new();
  let result = runtime.run(node);
  match result {
    Err(_) | Ok(_) => (),
  }
  let start_main = Node::FunctionCall{name: "main".to_string(), children: vec![]};
  let result = runtime.run(&start_main);
  match &result {
    Err(_) | Ok(_) => {
        return result;
    }
  }
}